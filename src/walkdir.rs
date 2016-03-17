use std::ffi::OsStr;
use std::fmt;
use std::fs::Metadata;
use std::io::Error;
use std::path::Path;
use fts::{Fts, FtsEntry, FtsInfo, fts_option};

// ---------------------------------------------------------------------------------------------------------------------
// DirEntry
// ---------------------------------------------------------------------------------------------------------------------

pub struct DirEntry {
    ent: FtsEntry,
}

impl DirEntry {
    /// Returns the full path to the file that this entry represents.
    ///
    /// The full path is created by joining the original path to `WalkDir::new` with the filename of this entry.
    pub fn path( &self ) -> &Path {
        &self.ent.path
    }

    /// Return the metadata for the file that this entry points at.
    pub fn metadata( &self ) -> Option<Metadata> {
        if self.ent.stat.is_some() {
            Some( self.ent.stat.clone().unwrap() )
        } else {
            None
        }
    }

    /// Return the file type for the file that this entry points at.
    pub fn file_type( &self ) -> FileType {
        FileType{ info: self.ent.info.clone() }
    }

    /// Returns the bare file name of this directory entry without any other leading path component.
    pub fn file_name( &self ) -> &OsStr {
        self.ent.name.as_os_str()
    }

    /// Returns the depth at which this entry was created relative to the original path to `WalkDir::new`.
    pub fn depth( &self ) -> usize {
        self.ent.level as usize
    }
}

impl fmt::Debug for DirEntry {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{:?}", self.ent )
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// FileType
// ---------------------------------------------------------------------------------------------------------------------

pub struct FileType {
    info: FtsInfo,
}

impl FileType {
    /// Test whether this file type represents a directory.
    pub fn is_dir( &self ) -> bool {
        self.info == FtsInfo::IsDir       ||
        self.info == FtsInfo::IsDirCyclic ||
        self.info == FtsInfo::IsDirPost
    }

    /// Test whether this file type represents a regular file.
    pub fn is_file( &self ) -> bool {
        self.info == FtsInfo::IsFile
    }

    /// Test whether this file type represents a symbolic link.
    pub fn is_symlink( &self ) -> bool {
        self.info == FtsInfo::IsSymlink     ||
        self.info == FtsInfo::IsSymlinkNone
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Iter
// ---------------------------------------------------------------------------------------------------------------------

pub struct Iter {
    fts: Fts,
}

impl Iterator for Iter {
    type Item = Result<DirEntry, Error>;

    fn next( &mut self ) -> Option<Result<DirEntry, Error>> {
        let ret = self.fts.read();
        if ret.is_some() {
            let ent = ret.unwrap();
            if ent.info == FtsInfo::IsErr || ent.info == FtsInfo::IsDontRead {
                Some( Err( Error::from_raw_os_error( ent.error ) ) )
            } else {
                Some( Ok( DirEntry{ ent: ent } ) )
            }
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// WalkDirConf
// ---------------------------------------------------------------------------------------------------------------------

pub struct WalkDirConf {
    path          : String,
    follow_symlink: bool  ,
    cross_device  : bool  ,
    include_dot   : bool  ,
    no_metadata   : bool  ,
    no_chdir      : bool  ,
}

impl WalkDirConf {
    /// Create new `WalkDirConf` with the root directory for directory walking.
    pub fn new<P: AsRef<Path>>( root: P ) -> Self {
        let path = root.as_ref().to_str().unwrap();

        WalkDirConf{
            path          : String::from( path ),
            follow_symlink: false,
            cross_device  : false,
            include_dot   : false,
            no_metadata   : false,
            no_chdir      : false,
        }
    }

    /// Enable following symblic links.
    pub fn follow_symlink( mut self ) -> Self {
        self.follow_symlink = true;
        self
    }

    /// Enable following symblic links across devices .
    pub fn cross_device( mut self ) -> Self {
        self.cross_device = true;
        self
    }

    /// Enable enumerating `.` and `..`.
    pub fn include_dot( mut self ) -> Self {
        self.include_dot = true;
        self
    }

    /// Disable providing metadata.
    pub fn no_metadata( mut self ) -> Self {
        self.no_metadata = true;
        self
    }

    /// Disable changing current directory through directory walking.
    pub fn no_chdir( mut self ) -> Self {
        self.no_chdir = true;
        self
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// WalkDir
// ---------------------------------------------------------------------------------------------------------------------

/// A builder to create an iterator for directory walking.
///
/// # Examples
/// The simplest usage is the following.
///
/// ```
/// # use std::path::Path;
/// # use fts::walkdir::{WalkDir, WalkDirConf};
/// let path = Path::new( "test" );
/// for p in WalkDir::new( WalkDirConf::new( path ) ) {
///     println!( "{:?}", p.unwrap() );
/// }
/// ```
///
/// `WalkDirConf` is a configuration builder of directory walking.
/// For example, if you want to follow symblic links, you can use `follow_symlink()` like the following.
///
/// ```
/// # use std::path::Path;
/// # use fts::walkdir::{WalkDir, WalkDirConf};
/// let path = Path::new( "test" );
/// for p in WalkDir::new( WalkDirConf::new( path ).follow_symlink() ) {
///     println!( "{:?}", p.unwrap() );
/// }
/// ```
///
/// If you want to use metadata of files, you can use `no_metadata()` for performance optimization like the following.
///
/// ```
/// # use std::path::Path;
/// # use fts::walkdir::{WalkDir, WalkDirConf};
/// let path = Path::new( "test" );
/// for p in WalkDir::new( WalkDirConf::new( path ).no_metadata() ) {
///     println!( "{:?}", p.unwrap() );
/// }
/// ```
///
pub struct WalkDir {
    conf: WalkDirConf,
    fts : Fts        ,
}

impl WalkDir {
    /// Create new `WalkDir` configured by specified `WalkDirConf`.
    pub fn new( conf: WalkDirConf ) -> Self {
        let mut option = if conf.follow_symlink { fts_option::LOGICAL } else { fts_option::PHYSICAL };
        option = if conf.cross_device { option | fts_option::XDEV    } else { option };
        option = if conf.include_dot  { option | fts_option::SEEDOT  } else { option };
        option = if conf.no_metadata  { option | fts_option::NOSTAT  } else { option };
        option = if conf.no_chdir     { option | fts_option::NOCHDIR } else { option };

        let path = conf.path.clone();
        WalkDir{
            conf: conf,
            fts : Fts::new( vec![path], option ).unwrap(),
        }
    }

    /// Return the base directory for directory walking.
    pub fn path             ( &self ) -> &str { &self.conf.path          }
    /// Test whether `WalkDir` follows symblic links.
    pub fn is_follow_symlink( &self ) -> bool { self.conf.follow_symlink }
    /// Test whether `WalkDir` follows symblic links across devices.
    pub fn is_cross_device  ( &self ) -> bool { self.conf.cross_device   }
    /// Test whether `WalkDir` enumerates `.` and `..`.
    pub fn is_include_dot   ( &self ) -> bool { self.conf.include_dot    }
    /// Test whether `WalkDir` provides metadata.
    pub fn is_no_metadata   ( &self ) -> bool { self.conf.no_metadata    }
    /// Test whether `WalkDir` change current directory through directory walking.
    pub fn is_no_chdir      ( &self ) -> bool { self.conf.no_chdir       }
}

impl IntoIterator for WalkDir {
    type Item = Result<DirEntry, Error>;
    type IntoIter = Iter;

    fn into_iter( self ) -> Iter {
        Iter {
            fts: self.fts,
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn normal() {
        let path = Path::new( "test" );
        for p in WalkDir::new( WalkDirConf::new( path ) ) {
            println!( "{:?}", p.unwrap() );
        }
    }

    #[test]
    fn filter() {
        let path = Path::new( "test" );
        let iter = WalkDir::new( WalkDirConf::new( path ) ).into_iter().filter_map( |x| x.ok() );
        for p in iter.filter( |x| x.file_type().is_file() ) {
            println!( "{:?}", p );
        }
    }
}

