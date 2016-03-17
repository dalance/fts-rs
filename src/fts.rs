use ffi;
use std::ffi::{CString, OsStr};
use std::fmt;
use std::fs::Metadata;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::{mem, ptr, slice};
use libc::stat;
use num::FromPrimitive;

pub mod fts_option {
    bitflags! {
        pub flags Flags: u32 {
            /// follow command line symlinks
            const COMFOLLOW = 0x0001,
            /// logical walk
            const LOGICAL   = 0x0002,
            /// don't change directories
            const NOCHDIR   = 0x0004,
            /// don't get stat info
            const NOSTAT    = 0x0008,
            /// physical walk
            const PHYSICAL  = 0x0010,
            /// return dot and dot-dot
            const SEEDOT    = 0x0020,
            /// don't cross devices
            const XDEV      = 0x0040,
        }
    }
}

#[derive(Debug)]
pub enum FtsSetOption {
    /// read node again
    Again  = ffi::FTS_AGAIN  as isize,
    /// follow symbolic link
    Follow = ffi::FTS_FOLLOW as isize,
    /// discard node
    Skip   = ffi::FTS_SKIP   as isize,
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
    SetFail     ,
}

pub struct Fts {
    fts: *mut ffi::FTS,
    opt: fts_option::Flags,
}

pub struct FtsEntry {
    pub path : PathBuf ,
    pub name : PathBuf ,
    pub info : FtsInfo ,
    pub stat : Option<Metadata>,
    pub level: i32     ,
    pub error: i32     ,
    ptr      : *const ffi::FTSENT,
}

impl fmt::Debug for FtsEntry {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        let len ;
        let perm;
        if self.stat.is_some() {
            let stat = self.stat.clone().unwrap();
            len  = stat.len();
            perm = format!( "{:?}", stat.permissions() );
        } else {
            len  = 0;
            perm = String::from( "Unknown" );
        }

        write!( f, "{{ path: {:?}, name: {:?}, info: {:?}, len: {}, perm: {}, level: {}, error: {} }}",
                self.path,
                self.name,
                self.info,
                len,
                perm,
                self.level,
                self.error
        )
    }
}


impl Fts {
    pub fn new( paths: Vec<String>, option: fts_option::Flags ) -> Result<Self, FtsError> {
        let mut c_paths = Vec::new();
        for p in paths {
            match CString::new( p ) {
                Ok ( p ) => c_paths.push( p.as_ptr() ),
                Err( _ ) => return Err( FtsError::PathWithNull ),
            }
        }
        c_paths.push( ptr::null() );
        let fts = unsafe { ffi::fts_open( c_paths.as_ptr(), option.bits() as i32, None ) };
        Ok( Fts { fts: fts, opt: option } )
    }

    pub fn read( &mut self ) -> Option<FtsEntry> {
        let ent = unsafe { ffi::fts_read( self.fts ) };
        let is_no_stat = self.opt.contains( fts_option::NOSTAT );

        Fts::to_fts_entry( ent, is_no_stat )
    }

    pub fn set( &mut self, ent: &FtsEntry, option: FtsSetOption ) -> Result<(), FtsError> {
        let ret = unsafe { ffi::fts_set( self.fts, ent.ptr, option as i32 ) };
        match ret {
            0 => Ok ( () ),
            _ => Err( FtsError::SetFail ),
        }
    }

    fn to_path( buf: *mut u8, len: usize ) -> PathBuf {
        unsafe {
            let slice = slice::from_raw_parts( buf, len );
            let osstr = OsStr::from_bytes( slice );
            Path::new( osstr ).to_path_buf()
        }
    }

    fn to_fts_entry( ent: *const ffi::FTSENT, is_no_stat: bool ) -> Option<FtsEntry> {
        if ent.is_null() {
            return None;
        }

        let len  = unsafe { (*ent).fts_namelen as usize };
        let ptr  = unsafe { mem::transmute::<&[i8;1], *mut u8>( &(*ent).fts_name ) };
        let name = Fts::to_path( ptr, len );

        let len  = unsafe { (*ent).fts_pathlen as usize };
        let ptr  = unsafe { (*ent).fts_path as *mut u8 };
        let path = Fts::to_path( ptr, len );

        let info  = unsafe { (*ent).fts_info as isize };
        let level = unsafe { (*ent).fts_level as i32 };
        let error = unsafe { (*ent).fts_errno as i32 };
        let stat  = unsafe {
            if is_no_stat {
                None
            } else {
                Some( (*mem::transmute::<*const stat, *const Metadata>( (*ent).fts_statp )).clone() )
            }
        };

        Some( FtsEntry {
            name : name ,
            path : path ,
            info : FtsInfo::from_isize( info ).unwrap_or( FtsInfo::IsUnknown ),
            stat : stat,
            level: level,
            error: error,
            ptr  : ent  ,
        } )
    }
}

impl Drop for Fts {
    fn drop( &mut self ) {
        unsafe {
            ffi::fts_close( self.fts );
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{Permissions, set_permissions};
    use std::io;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    fn check_entry( entry: FtsEntry, is_logical: bool ) {
        if entry.path == PathBuf::from( "test" ) {
            assert!( entry.info == FtsInfo::IsDir || entry.info == FtsInfo::IsDirPost );
            assert_eq!( entry.level, 0 );
        }
        if entry.path == PathBuf::from( "test/file" ) {
            assert!( entry.info == FtsInfo::IsFile );
            assert_eq!( entry.level, 1 );
        }
        if entry.path == PathBuf::from( "test/dir" ) {
            assert!( entry.info == FtsInfo::IsDir || entry.info == FtsInfo::IsDirPost );
            assert_eq!( entry.level, 1 );
        }
        if entry.path == PathBuf::from( "test/dir/file" ) {
            assert!( entry.info == FtsInfo::IsFile );
            assert_eq!( entry.level, 2 );
        }
        if entry.path == PathBuf::from( "test/link_file" ) {
            if is_logical {
                assert!( entry.info == FtsInfo::IsFile );
            } else {
                assert!( entry.info == FtsInfo::IsSymlink );
            }
            assert_eq!( entry.level, 1 );
        }
        if entry.path == PathBuf::from( "test/link_none" ) {
            if is_logical {
                assert!( entry.info == FtsInfo::IsSymlinkNone );
            } else {
                assert!( entry.info == FtsInfo::IsSymlink );
            }
            assert_eq!( entry.level, 1 );
        }
        if entry.path == PathBuf::from( "test/cyclic" ) {
            assert!( entry.info == FtsInfo::IsDir || entry.info == FtsInfo::IsDirPost );
            assert_eq!( entry.level, 1 );
        }
        if entry.path == PathBuf::from( "test/cyclic/cyclic" ) {
            if is_logical {
                assert!( entry.info == FtsInfo::IsDirCyclic );
            } else {
                assert!( entry.info == FtsInfo::IsSymlink );
            }
            assert_eq!( entry.level, 2 );
        }
        if entry.path == PathBuf::from( "test/dir2" ) {
            assert!( entry.info == FtsInfo::IsDir || entry.info == FtsInfo::IsDirPost || entry.info == FtsInfo::IsDontRead );
            if entry.info == FtsInfo::IsDontRead {
                assert_eq!( io::Error::from_raw_os_error( entry.error ).kind(), io::ErrorKind::PermissionDenied );
            }
            assert_eq!( entry.level, 1 );
        }
    }

    #[test]
    fn logical() {
        let _ = set_permissions( "test/dir2", Permissions::from_mode( 0 ) );

        let paths = vec![String::from( "test" )];
        let mut fts = Fts::new( paths, fts_option::LOGICAL ).unwrap();

        let mut ftsent = fts.read();
        let mut i = 0;
        while ftsent.is_some() {
            let ent = ftsent.unwrap();
            check_entry( ent, true );
            ftsent = fts.read();
            i += 1;
        }
        assert_eq!( i, 13 );

        let _ = set_permissions( "test/dir2", Permissions::from_mode( 0o755 ) );
    }

    #[test]
    fn physical() {
        let _ = set_permissions( "test/dir2", Permissions::from_mode( 0 ) );

        let paths = vec![String::from( "test" )];
        let mut fts = Fts::new( paths, fts_option::PHYSICAL ).unwrap();

        let mut ftsent = fts.read();
        let mut i = 0;
        while ftsent.is_some() {
            let ent = ftsent.unwrap();
            check_entry( ent, false );
            ftsent = fts.read();
            i += 1;
        }
        assert_eq!( i, 13 );

        let _ = set_permissions( "test/dir2", Permissions::from_mode( 0o755 ) );
    }
}
