//! A wrapper library of libc fts.

use ffi;
use libc::{c_int, c_long, stat};
use num::FromPrimitive;
use std::cmp::Ordering;
use std::ffi::{CString, OsStr};
use std::fmt;
use std::fs::Metadata;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::{mem, ptr, slice};

// ---------------------------------------------------------------------------------------------------------------------
// enum
// ---------------------------------------------------------------------------------------------------------------------

pub mod fts_option {
    bitflags! {
        pub struct Flags: u32 {
            /// follow command line symlinks
            const COMFOLLOW = 0x0001;
            /// logical walk
            const LOGICAL   = 0x0002;
            /// don't change directories
            const NOCHDIR   = 0x0004;
            /// don't get stat info
            const NOSTAT    = 0x0008;
            /// physical walk
            const PHYSICAL  = 0x0010;
            /// return dot and dot-dot
            const SEEDOT    = 0x0020;
            /// don't cross devices
            const XDEV      = 0x0040;
        }
    }
}

#[derive(Debug)]
pub enum FtsSetOption {
    /// read node again
    Again = ffi::FTS_AGAIN as isize,
    /// follow symbolic link
    Follow = ffi::FTS_FOLLOW as isize,
    /// discard node
    Skip = ffi::FTS_SKIP as isize,
}

enum_from_primitive! {
    #[derive(Clone,Debug,PartialEq)]
    pub enum FtsInfo {
        IsDir         = ffi::FTS_D       as isize,
        IsDirCyclic   = ffi::FTS_DC      as isize,
        IsDefault     = ffi::FTS_DEFAULT as isize,
        IsDontRead    = ffi::FTS_DNR     as isize,
        IsDot         = ffi::FTS_DOT     as isize,
        IsDirPost     = ffi::FTS_DP      as isize,
        IsErr         = ffi::FTS_ERR     as isize,
        IsFile        = ffi::FTS_F       as isize,
        IsNoStat      = ffi::FTS_NS      as isize,
        IsNoStatOk    = ffi::FTS_NSOK    as isize,
        IsSymlink     = ffi::FTS_SL      as isize,
        IsSymlinkNone = ffi::FTS_SLNONE  as isize,
        IsUnknown,
    }
}

#[derive(Debug)]
pub enum FtsError {
    /// path string contains null charactors.
    PathWithNull,
    /// fts_set() failed.
    SetFail,
}

// ---------------------------------------------------------------------------------------------------------------------
// FtsEntry
// ---------------------------------------------------------------------------------------------------------------------

pub struct FtsEntry {
    pub path: PathBuf,
    pub name: PathBuf,
    pub info: FtsInfo,
    pub stat: Option<Metadata>,
    pub level: i32,
    pub error: i32,
    ptr: *const ffi::FTSENT,
}

impl fmt::Debug for FtsEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len;
        let perm;
        if self.stat.is_some() {
            let stat = self.stat.clone().unwrap();
            len = stat.len();
            perm = format!("{:?}", stat.permissions());
        } else {
            len = 0;
            perm = String::from("Unknown");
        }

        write!(
            f,
            "{{ path: {:?}, name: {:?}, info: {:?}, len: {}, perm: {}, level: {}, error: {} }}",
            self.path, self.name, self.info, len, perm, self.level, self.error
        )
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Fts
// ---------------------------------------------------------------------------------------------------------------------

pub struct Fts {
    fts: *mut ffi::FTS,
    opt: fts_option::Flags,
}

impl Fts {
    pub fn new(
        paths: Vec<String>,
        option: fts_option::Flags,
        cmp: Option<FtsCompFunc>,
    ) -> Result<Self, FtsError> {
        // c_paths holding memory until the end of function.
        let mut c_paths = Vec::new();
        let mut c_path_ptrs = Vec::new();
        for p in paths {
            match CString::new(p) {
                Ok(p) => {
                    c_path_ptrs.push(p.as_ptr());
                    c_paths.push(p);
                }
                Err(_) => return Err(FtsError::PathWithNull),
            }
        }
        c_path_ptrs.push(ptr::null());

        let fts = unsafe { ffi::fts_open(c_path_ptrs.as_ptr(), option.bits() as i32, cmp) };
        assert!(!fts.is_null());

        Ok(Fts {
            fts: fts,
            opt: option,
        })
    }

    pub fn read(&mut self) -> Option<FtsEntry> {
        let ent = unsafe { ffi::fts_read(self.fts) };
        let is_no_stat = self.opt.contains(fts_option::Flags::NOSTAT);

        Fts::to_fts_entry(ent, is_no_stat)
    }

    pub fn set(&mut self, ent: &FtsEntry, option: FtsSetOption) -> Result<(), FtsError> {
        let ret = unsafe { ffi::fts_set(self.fts, ent.ptr, option as i32) };
        match ret {
            0 => Ok(()),
            _ => Err(FtsError::SetFail),
        }
    }

    fn to_path(buf: *const u8, len: usize) -> PathBuf {
        let slice = unsafe { slice::from_raw_parts(buf, len) };
        let osstr = OsStr::from_bytes(slice);
        Path::new(osstr).to_path_buf()
    }

    fn to_fts_entry(ent: *const ffi::FTSENT, is_no_stat: bool) -> Option<FtsEntry> {
        if ent.is_null() {
            return None;
        }

        let len = unsafe { (*ent).fts_namelen as usize };
        let ptr = unsafe { (&(*ent).fts_name) as *const u8 };
        let name = Fts::to_path(ptr, len);

        let len = unsafe { (*ent).fts_pathlen as usize };
        let ptr = unsafe { (*ent).fts_path as *const u8 };
        let path = Fts::to_path(ptr, len);

        let info = unsafe { (*ent).fts_info as isize };
        let level = unsafe { (*ent).fts_level as i32 };
        let error = unsafe { (*ent).fts_errno as i32 };
        let stat = unsafe {
            if is_no_stat {
                None
            } else {
                Some((*mem::transmute::<*const stat, *const Metadata>((*ent).fts_statp)).clone())
            }
        };

        Some(FtsEntry {
            name: name,
            path: path,
            info: FtsInfo::from_isize(info).unwrap_or(FtsInfo::IsUnknown),
            stat: stat,
            level: level,
            error: error,
            ptr: ent,
        })
    }
}

impl Drop for Fts {
    fn drop(&mut self) {
        unsafe {
            ffi::fts_close(self.fts);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// FtsComp
// ---------------------------------------------------------------------------------------------------------------------

pub type FtsCompIn = *const *const ffi::FTSENT;
pub type FtsCompFunc = extern "C" fn(FtsCompIn, FtsCompIn) -> c_int;

pub struct FtsComp;

impl FtsComp {
    pub extern "C" fn by_name_ascending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_ascending_ref(ent0, ent1, FtsComp::to_name)
    }

    pub extern "C" fn by_name_descending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_descending_ref(ent0, ent1, FtsComp::to_name)
    }

    pub extern "C" fn by_atime_ascending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_ascending_val(ent0, ent1, FtsComp::to_atime)
    }

    pub extern "C" fn by_atime_descending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_descending_val(ent0, ent1, FtsComp::to_atime)
    }

    pub extern "C" fn by_mtime_ascending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_ascending_val(ent0, ent1, FtsComp::to_mtime)
    }

    pub extern "C" fn by_mtime_descending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_descending_val(ent0, ent1, FtsComp::to_mtime)
    }

    pub extern "C" fn by_ctime_ascending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_ascending_val(ent0, ent1, FtsComp::to_ctime)
    }

    pub extern "C" fn by_ctime_descending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_descending_val(ent0, ent1, FtsComp::to_ctime)
    }

    pub extern "C" fn by_len_ascending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_ascending_val(ent0, ent1, FtsComp::to_len)
    }

    pub extern "C" fn by_len_descending(ent0: FtsCompIn, ent1: FtsCompIn) -> c_int {
        FtsComp::by_descending_val(ent0, ent1, FtsComp::to_len)
    }

    fn by_ascending_val<T: PartialOrd>(
        ent0: FtsCompIn,
        ent1: FtsCompIn,
        to_value: fn(FtsCompIn) -> T,
    ) -> c_int {
        let val0 = to_value(ent0);
        let val1 = to_value(ent1);
        match val0.partial_cmp(&val1) {
            Some(Ordering::Less) => -1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            None => 0,
        }
    }

    fn by_ascending_ref<'a, T: PartialOrd + ?Sized>(
        ent0: FtsCompIn,
        ent1: FtsCompIn,
        to_value: fn(FtsCompIn) -> &'a T,
    ) -> c_int {
        let val0 = to_value(ent0);
        let val1 = to_value(ent1);
        match val0.partial_cmp(&val1) {
            Some(Ordering::Less) => -1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => 1,
            None => 0,
        }
    }

    fn by_descending_val<T: PartialOrd>(
        ent0: FtsCompIn,
        ent1: FtsCompIn,
        to_value: fn(FtsCompIn) -> T,
    ) -> c_int {
        let val0 = to_value(ent0);
        let val1 = to_value(ent1);
        match val0.partial_cmp(&val1) {
            Some(Ordering::Less) => 1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => -1,
            None => 0,
        }
    }

    fn by_descending_ref<'a, T: PartialOrd + ?Sized>(
        ent0: FtsCompIn,
        ent1: FtsCompIn,
        to_value: fn(FtsCompIn) -> &'a T,
    ) -> c_int {
        let val0 = to_value(ent0);
        let val1 = to_value(ent1);
        match val0.partial_cmp(&val1) {
            Some(Ordering::Less) => 1,
            Some(Ordering::Equal) => 0,
            Some(Ordering::Greater) => -1,
            None => 0,
        }
    }

    fn to_name<'a>(ent: *const *const ffi::FTSENT) -> &'a OsStr {
        let len = unsafe { (**ent).fts_namelen as usize };
        let ptr = unsafe { (&(**ent).fts_name) as *const u8 };
        FtsComp::to_osstr(ptr, len)
    }

    fn to_osstr<'a>(buf: *const u8, len: usize) -> &'a OsStr {
        let slice = unsafe { slice::from_raw_parts(buf, len) };
        OsStr::from_bytes(slice)
    }

    fn to_atime(ent: *const *const ffi::FTSENT) -> c_long {
        let statp = unsafe { (**ent).fts_statp };
        assert!(!statp.is_null());
        unsafe { (*mem::transmute::<*const stat, *const Metadata>((**ent).fts_statp)).atime_nsec() }
    }

    fn to_mtime(ent: *const *const ffi::FTSENT) -> c_long {
        let statp = unsafe { (**ent).fts_statp };
        assert!(!statp.is_null());
        unsafe { (*mem::transmute::<*const stat, *const Metadata>((**ent).fts_statp)).mtime_nsec() }
    }

    fn to_ctime(ent: *const *const ffi::FTSENT) -> c_long {
        let statp = unsafe { (**ent).fts_statp };
        assert!(!statp.is_null());
        unsafe { (*mem::transmute::<*const stat, *const Metadata>((**ent).fts_statp)).ctime_nsec() }
    }

    fn to_len(ent: *const *const ffi::FTSENT) -> u64 {
        let statp = unsafe { (**ent).fts_statp };
        assert!(!statp.is_null());
        unsafe { (*mem::transmute::<*const stat, *const Metadata>((**ent).fts_statp)).len() }
    }
}

// ---------------------------------------------------------------------------------------------------------------------
// Test
// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{set_permissions, Permissions};
    use std::io;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    fn check_entry(entry: FtsEntry, is_logical: bool) {
        if entry.path == PathBuf::from("test_data") {
            assert!(entry.info == FtsInfo::IsDir || entry.info == FtsInfo::IsDirPost);
            assert_eq!(entry.level, 0);
        }
        if entry.path == PathBuf::from("test_data/file") {
            assert!(entry.info == FtsInfo::IsFile);
            assert_eq!(entry.level, 1);
        }
        if entry.path == PathBuf::from("test_data/dir") {
            assert!(entry.info == FtsInfo::IsDir || entry.info == FtsInfo::IsDirPost);
            assert_eq!(entry.level, 1);
        }
        if entry.path == PathBuf::from("test_data/dir/file") {
            assert!(entry.info == FtsInfo::IsFile);
            assert_eq!(entry.level, 2);
        }
        if entry.path == PathBuf::from("test_data/link_file") {
            if is_logical {
                assert!(entry.info == FtsInfo::IsFile);
            } else {
                assert!(entry.info == FtsInfo::IsSymlink);
            }
            assert_eq!(entry.level, 1);
        }
        if entry.path == PathBuf::from("test_data/link_none") {
            if is_logical {
                assert!(entry.info == FtsInfo::IsSymlinkNone);
            } else {
                assert!(entry.info == FtsInfo::IsSymlink);
            }
            assert_eq!(entry.level, 1);
        }
        if entry.path == PathBuf::from("test_data/cyclic") {
            assert!(entry.info == FtsInfo::IsDir || entry.info == FtsInfo::IsDirPost);
            assert_eq!(entry.level, 1);
        }
        if entry.path == PathBuf::from("test_data/cyclic/cyclic") {
            if is_logical {
                assert!(entry.info == FtsInfo::IsDirCyclic);
            } else {
                assert!(entry.info == FtsInfo::IsSymlink);
            }
            assert_eq!(entry.level, 2);
        }
        if entry.path == PathBuf::from("test_data/dir2") {
            assert!(
                entry.info == FtsInfo::IsDir
                    || entry.info == FtsInfo::IsDirPost
                    || entry.info == FtsInfo::IsDontRead
            );
            if entry.info == FtsInfo::IsDontRead {
                assert_eq!(
                    io::Error::from_raw_os_error(entry.error).kind(),
                    io::ErrorKind::PermissionDenied
                );
            }
            assert_eq!(entry.level, 1);
        }
    }

    #[test]
    fn logical() {
        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0));

        let paths = vec![String::from("test_data")];
        let mut fts = Fts::new(paths, fts_option::Flags::LOGICAL, None).unwrap();

        let mut ftsent = fts.read();
        let mut i = 0;
        while ftsent.is_some() {
            let ent = ftsent.unwrap();
            check_entry(ent, true);
            ftsent = fts.read();
            i += 1;
        }
        assert_eq!(i, 23);

        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0o755));
    }

    #[test]
    fn physical() {
        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0));

        let paths = vec![String::from("test_data")];
        let mut fts = Fts::new(paths, fts_option::Flags::PHYSICAL, None).unwrap();

        let mut ftsent = fts.read();
        let mut i = 0;
        while ftsent.is_some() {
            let ent = ftsent.unwrap();
            check_entry(ent, false);
            ftsent = fts.read();
            i += 1;
        }
        assert_eq!(i, 23);

        let _ = set_permissions("test_data/dir2", Permissions::from_mode(0o755));
    }

    #[test]
    fn sort() {
        let paths = vec![String::from("test_data/sort")];
        let mut fts = Fts::new(
            paths,
            fts_option::Flags::LOGICAL,
            Some(FtsComp::by_name_ascending),
        )
        .unwrap();

        let mut ftsent = fts.read();
        while ftsent.is_some() {
            let ent = ftsent.unwrap();
            check_entry(ent, true);
            ftsent = fts.read();
        }
    }

    #[test]
    fn path_with_null() {
        let paths = vec![String::from("test_data\0/sort")];
        let fts = Fts::new(paths, fts_option::Flags::LOGICAL, None);
        match fts {
            Err(FtsError::PathWithNull) => assert!(true),
            _ => assert!(false),
        }
    }
}
