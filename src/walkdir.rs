//! A library for directory walking.
//!
//! # Examples
//! The simplest usage is the following.
//!
//! ```
//! # use std::path::Path;
//! # use fts::walkdir::{WalkDir, WalkDirConf};
//! let path = Path::new( "test_data" );
//! for p in WalkDir::new( WalkDirConf::new( path ) ) {
//!     println!( "{:?}", p.unwrap() );
//! }
//! ```
//!
//! `WalkDirConf` is a configuration builder of directory walking.
//! For example, if you want to follow symblic links, you can use `follow_symlink()` like the following.
//!
//! ```
//! # use std::path::Path;
//! # use fts::walkdir::{WalkDir, WalkDirConf};
//! let path = Path::new( "test_data" );
//! for p in WalkDir::new( WalkDirConf::new( path ).follow_symlink() ) {
//!     println!( "{:?}", p.unwrap() );
//! }
//! ```
//!
//! If you don't want to use metadata of files, you can use `no_metadata()` for performance optimization like the following.
//!
//! ```
//! # use std::path::Path;
//! # use fts::walkdir::{WalkDir, WalkDirConf};
//! let path = Path::new( "test_data" );
//! for p in WalkDir::new( WalkDirConf::new( path ).no_metadata() ) {
//!     println!( "{:?}", p.unwrap() );
//! }
//! ```
//!
//! If you want to enumerate directories sorted by the creation time of file, you can use `sort_by_ctime()`.
//! `sort_ascending()` means sorting in ascending order, and `sort_descending()` means descending order.
//!
//! ```
//! # use std::path::Path;
//! # use fts::walkdir::{WalkDir, WalkDirConf};
//! let path = Path::new( "test_data" );
//! for p in WalkDir::new( WalkDirConf::new( path ).sort_by_ctime().sort_ascending() ) {
//!     println!( "{:?}", p.unwrap() );
//! }
//! ```
//!

use fts::{fts_option, Fts, FtsComp, FtsCompFunc, FtsEntry, FtsInfo};
use std::ffi::OsStr;
use std::fmt;
use std::fs::Metadata;
use std::io::Error;
use std::path::Path;

// ---------------------------------------------------------------------------------------------------------------------
// DirEntry
// ---------------------------------------------------------------------------------------------------------------------

/// A directory entry like `std::fs::DirEntry`.
pub struct DirEntry {
    ent: FtsEntry,
}

impl DirEntry {
    /// Returns the full path to the file that this entry represents.
    ///
    /// The full path is created by joining the original path to `WalkDir::new` with the filename of this entry.
    pub fn path(&self) -> &Path {
        &self.ent.path
    }

    /// Return the metadata for the file that this entry points at.
    pub fn metadata(&self) -> Option<Metadata> {
        if self.ent.stat.is_some() {
            Some(self.ent.stat.clone().unwrap())
        } else {
            None
        }
    }

    /// Return the file type for the file that this entry points at.
    pub fn file_type(&self) -> FileType {
        FileType {
            info: self.ent.info.clone(),
        }
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    pub fn file_name(&self) -> &OsStr {
        self.ent.name.as_os_str()
    }

    /// Returns the depth at which this entry was created relative to the original path to `WalkDir::new`.
    pub fn depth(&self) -> usize {
        self.ent.level as usize
    }
}

impl fmt::Debug for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.ent)
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// FileType
// ---------------------------------------------------------------------------------------------------------------------

/// A file type of the directory entry like `std::fs::FileType`.
pub struct FileType {
    info: FtsInfo,
}

impl FileType {
    /// Test whether this file type represents a directory.
    pub fn is_dir(&self) -> bool {
        self.info == FtsInfo::IsDir
            || self.info == FtsInfo::IsDirCyclic
            || self.info == FtsInfo::IsDirPost
    }

    /// Test whether this file type represents a regular file.
    pub fn is_file(&self) -> bool {
        self.info == FtsInfo::IsFile
    }

    /// Test whether this file type represents a symbolic link.
    pub fn is_symlink(&self) -> bool {
        self.info == FtsInfo::IsSymlink || self.info == FtsInfo::IsSymlinkNone
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Iter
// ---------------------------------------------------------------------------------------------------------------------

/// A iterator for enumerating directory entries.
pub struct Iter {
    fts: Fts,
}

impl Iterator for Iter {
    type Item = Result<DirEntry, Error>;

    fn next(&mut self) -> Option<Result<DirEntry, Error>> {
        let ret = self.fts.read();
        if ret.is_some() {
            let ent = ret.unwrap();
            if ent.info == FtsInfo::IsErr
                || ent.info == FtsInfo::IsDontRead
                || ent.info == FtsInfo::IsNoStat
            {
                Some(Err(Error::from_raw_os_error(ent.error)))
            } else {
                Some(Ok(DirEntry { ent: ent }))
            }
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// WalkDirConf
// ---------------------------------------------------------------------------------------------------------------------

#[derive(PartialEq)]
enum SortBy {
    None,
    Name,
    Len,
    ATime,
    MTime,
    CTime,
}

#[derive(PartialEq)]
enum SortDir {
    Ascending,
    Descending,
}

/// A configuration builder of the settings for directory walking.
pub struct WalkDirConf {
    path: String,
    follow_symlink: bool,
    cross_device: bool,
    include_dot: bool,
    no_metadata: bool,
    no_chdir: bool,
    sort_by: SortBy,
    sort_dir: SortDir,
}

impl WalkDirConf {
    /// Create new `WalkDirConf` with the root directory for directory walking.
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let path = root.as_ref().to_str().unwrap();

        WalkDirConf {
            path: String::from(path),
            follow_symlink: false,
            cross_device: false,
            include_dot: false,
            no_metadata: false,
            no_chdir: false,
            sort_by: SortBy::None,
            sort_dir: SortDir::Ascending,
        }
    }

    /// Enable following symblic links.
    pub fn follow_symlink(mut self) -> Self {
        self.follow_symlink = true;
        self
    }

    /// Enable following symblic links across devices .
    pub fn cross_device(mut self) -> Self {
        self.cross_device = true;
        self
    }

    /// Enable enumerating `.` and `..`.
    pub fn include_dot(mut self) -> Self {
        self.include_dot = true;
        self
    }

    /// Disable providing metadata.
    pub fn no_metadata(mut self) -> Self {
        self.no_metadata = true;
        self
    }

    /// Disable changing current directory through directory walking.
    pub fn no_chdir(mut self) -> Self {
        self.no_chdir = true;
        self
    }

    /// Sort by file name.
    pub fn sort_by_name(mut self) -> Self {
        self.sort_by = SortBy::Name;
        self
    }

    /// Sort by file length.
    pub fn sort_by_len(mut self) -> Self {
        self.sort_by = SortBy::Len;
        self
    }

    /// Sort by access time.
    pub fn sort_by_atime(mut self) -> Self {
        self.sort_by = SortBy::ATime;
        self
    }

    /// Sort by create time.
    pub fn sort_by_ctime(mut self) -> Self {
        self.sort_by = SortBy::CTime;
        self
    }

    /// Sort by modify time.
    pub fn sort_by_mtime(mut self) -> Self {
        self.sort_by = SortBy::MTime;
        self
    }

    /// Sort by ascending order.
    pub fn sort_ascending(mut self) -> Self {
        self.sort_dir = SortDir::Ascending;
        self
    }

    /// Sort by descending order.
    pub fn sort_descending(mut self) -> Self {
        self.sort_dir = SortDir::Descending;
        self
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// WalkDir
// ---------------------------------------------------------------------------------------------------------------------

/// A builder to create an iterator for directory walking.
pub struct WalkDir {
    conf: WalkDirConf,
    fts: Fts,
}

impl WalkDir {
    /// Create new `WalkDir` configured by specified `WalkDirConf`.
    pub fn new(conf: WalkDirConf) -> Self {
        let mut option = if conf.follow_symlink {
            fts_option::Flags::LOGICAL
        } else {
            fts_option::Flags::PHYSICAL
        };
        option = if conf.cross_device {
            option | fts_option::Flags::XDEV
        } else {
            option
        };
        option = if conf.include_dot {
            option | fts_option::Flags::SEEDOT
        } else {
            option
        };
        option = if conf.no_metadata {
            option | fts_option::Flags::NOSTAT
        } else {
            option
        };
        option = if conf.no_chdir {
            option | fts_option::Flags::NOCHDIR
        } else {
            option
        };

        let is_ascending = conf.sort_dir == SortDir::Ascending;
        let is_descending = conf.sort_dir == SortDir::Descending;
        let is_metadata = !conf.no_metadata;

        let sorter = match conf.sort_by {
            SortBy::Name if is_ascending => Some(FtsComp::by_name_ascending as FtsCompFunc),
            SortBy::Name if is_descending => Some(FtsComp::by_name_descending as FtsCompFunc),
            SortBy::Len if is_ascending && is_metadata => {
                Some(FtsComp::by_len_ascending as FtsCompFunc)
            }
            SortBy::Len if is_descending && is_metadata => {
                Some(FtsComp::by_len_descending as FtsCompFunc)
            }
            SortBy::ATime if is_ascending && is_metadata => {
                Some(FtsComp::by_atime_ascending as FtsCompFunc)
            }
            SortBy::ATime if is_descending && is_metadata => {
                Some(FtsComp::by_atime_descending as FtsCompFunc)
            }
            SortBy::CTime if is_ascending && is_metadata => {
                Some(FtsComp::by_ctime_ascending as FtsCompFunc)
            }
            SortBy::CTime if is_descending && is_metadata => {
                Some(FtsComp::by_ctime_descending as FtsCompFunc)
            }
            SortBy::MTime if is_ascending && is_metadata => {
                Some(FtsComp::by_mtime_ascending as FtsCompFunc)
            }
            SortBy::MTime if is_descending && is_metadata => {
                Some(FtsComp::by_mtime_descending as FtsCompFunc)
            }
            _ => None,
        };

        let path = conf.path.clone();
        WalkDir {
            conf: conf,
            fts: Fts::new(vec![path], option, sorter).unwrap(),
        }
    }

    /// Return the base directory for directory walking.
    pub fn path(&self) -> &str {
        &self.conf.path
    }
    /// Test whether `WalkDir` follows symblic links.
    pub fn is_follow_symlink(&self) -> bool {
        self.conf.follow_symlink
    }
    /// Test whether `WalkDir` follows symblic links across devices.
    pub fn is_cross_device(&self) -> bool {
        self.conf.cross_device
    }
    /// Test whether `WalkDir` enumerates `.` and `..`.
    pub fn is_include_dot(&self) -> bool {
        self.conf.include_dot
    }
    /// Test whether `WalkDir` provides metadata.
    pub fn is_no_metadata(&self) -> bool {
        self.conf.no_metadata
    }
    /// Test whether `WalkDir` change current directory through directory walking.
    pub fn is_no_chdir(&self) -> bool {
        self.conf.no_chdir
    }
}

impl IntoIterator for WalkDir {
    type Item = Result<DirEntry, Error>;
    type IntoIter = Iter;

    fn into_iter(self) -> Iter {
        Iter { fts: self.fts }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{set_permissions, Permissions};
    use std::io::ErrorKind;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    #[test]
    fn normal() {
        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0));

        let path = Path::new("test_data");
        let iter = WalkDir::new(WalkDirConf::new(path))
            .into_iter()
            .filter_map(|x| x.ok());
        let mut cnt = 0;
        for _ in iter {
            cnt += 1;
        }
        assert_eq!(cnt, 22);

        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0o755));
    }

    #[test]
    fn filter() {
        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0));

        let path = Path::new("test_data");
        let iter = WalkDir::new(WalkDirConf::new(path))
            .into_iter()
            .filter_map(|x| x.ok());
        let mut cnt = 0;
        let mut len = 0;
        for p in iter.filter(|x| x.file_type().is_file()) {
            cnt += 1;
            len += p.metadata().unwrap().len();
        }
        assert_eq!(cnt, 10);
        assert_eq!(len, 150);

        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0o755));
    }

    #[test]
    fn no_stat() {
        let path = Path::new("test_data");
        let iter = WalkDir::new(WalkDirConf::new(path).no_metadata())
            .into_iter()
            .filter_map(|x| x.ok());
        for p in iter {
            assert!(p.metadata().is_none());
        }
    }

    #[test]
    fn dir_not_found() {
        let path = Path::new("aaa");
        for p in WalkDir::new(WalkDirConf::new(path)) {
            match p {
                Ok(_) => assert!(false),
                Err(x) => assert_eq!(x.kind(), ErrorKind::NotFound),
            }
        }
    }

    #[test]
    fn sort() {
        let path = Path::new("test_data/sort");
        {
            let conf = WalkDirConf::new(path).sort_by_name().sort_ascending();
            let mut iter = WalkDir::new(conf)
                .into_iter()
                .filter_map(|x| x.ok())
                .filter(|x| x.file_type().is_file());
            assert_eq!(iter.next().unwrap().file_name(), "0");
        }

        {
            let conf = WalkDirConf::new(path).sort_by_name().sort_descending();
            let mut iter = WalkDir::new(conf)
                .into_iter()
                .filter_map(|x| x.ok())
                .filter(|x| x.file_type().is_file());
            assert_eq!(iter.next().unwrap().file_name(), "d");
        }

        {
            let conf = WalkDirConf::new(path).sort_by_len().sort_ascending();
            let mut iter = WalkDir::new(conf)
                .into_iter()
                .filter_map(|x| x.ok())
                .filter(|x| x.file_type().is_file());
            assert_eq!(iter.next().unwrap().file_name(), "a");
        }

        {
            let conf = WalkDirConf::new(path).sort_by_len().sort_descending();
            let mut iter = WalkDir::new(conf)
                .into_iter()
                .filter_map(|x| x.ok())
                .filter(|x| x.file_type().is_file());
            assert_eq!(iter.next().unwrap().file_name(), "2");
        }
    }

    #[test]
    fn sort_time() {
        let path = Path::new("test_data/sort");
        {
            let conf = WalkDirConf::new(path).sort_by_atime().sort_ascending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path).sort_by_atime().sort_descending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path).sort_by_mtime().sort_ascending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path).sort_by_mtime().sort_descending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path).sort_by_ctime().sort_ascending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path).sort_by_ctime().sort_descending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path)
                .no_metadata()
                .sort_by_atime()
                .sort_ascending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path)
                .no_metadata()
                .sort_by_atime()
                .sort_descending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path)
                .no_metadata()
                .sort_by_mtime()
                .sort_ascending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path)
                .no_metadata()
                .sort_by_mtime()
                .sort_descending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path)
                .no_metadata()
                .sort_by_ctime()
                .sort_ascending();
            for _ in WalkDir::new(conf) {}
        }
        {
            let conf = WalkDirConf::new(path)
                .no_metadata()
                .sort_by_ctime()
                .sort_descending();
            for _ in WalkDir::new(conf) {}
        }
    }
}
