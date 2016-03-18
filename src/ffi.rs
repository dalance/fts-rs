use libc::*;

#[cfg(target_os="linux")]
pub type Inode = ino_t;

#[cfg(target_os="macos")]
pub type Inode = u32;

#[cfg(target_os="linux")]
pub type Stat = stat;

#[cfg(target_os="macos")]
#[repr(C)]
pub struct Stat {
    pub st_dev           : dev_t       ,
    pub st_mode          : mode_t      ,
    pub st_nlink         : nlink_t     ,
    pub st_ino           : ino_t       ,
    pub st_uid           : uid_t       ,
    pub st_gid           : gid_t       ,
    pub st_rdev          : dev_t       ,
    pub st_atime         : time_t      ,
    pub st_atime_nsec    : c_long      ,
    pub st_mtime         : time_t      ,
    pub st_mtime_nsec    : c_long      ,
    pub st_ctime         : time_t      ,
    pub st_ctime_nsec    : c_long      ,
    pub st_birthtime     : time_t      ,
    pub st_birthtime_nsec: c_long      ,
    pub st_size          : off_t       ,
    pub st_blocks        : blkcnt_t    ,
    pub st_blksize       : blksize_t   ,
    pub st_flags         : uint32_t    ,
    pub st_gen           : uint32_t    ,
    pub st_lspare        : int32_t     ,
    pub st_qspare        : [int64_t; 2],
}

/// struct FTS in fts.h ( opaque struct )
pub enum FTS {}

/// struct FTSENT in fts.h
#[repr(C)]
#[derive(Debug)]
pub struct FTSENT {
    /// cycle node
    pub fts_cycle  : *const FTSENT,
    /// parent directory
    pub fts_parent : *const FTSENT,
    /// next file in directory
    pub fts_link   : *const FTSENT,
    /// local numeric value
    pub fts_number : c_long       ,
    /// local address value
    pub fts_pointer: *const c_void,
    /// access path
    pub fts_accpath: *const c_char,
    /// root path
    pub fts_path   : *const c_char,
    /// errno for this node
    pub fts_errno  : c_int        ,
    /// fd for symlink
    pub fts_symfd  : c_int        ,
    /// strlen(fts_path)
    pub fts_pathlen: c_ushort     ,
    /// strlen(fts_name)
    pub fts_namelen: c_ushort     ,
    /// inode
    pub fts_ino    : Inode        ,
    /// device
    pub fts_dev    : dev_t        ,
    /// link count
    pub fts_nlink  : nlink_t      ,
    /// depth (-1 to N)
    pub fts_level  : c_short      ,
    /// user flags for FTSENT structure
    pub fts_info   : c_ushort     ,
    /// private flags for FTSENT structure
    pub fts_flags  : c_ushort     ,
    /// fts_set() instructions
    pub fts_instr  : c_ushort     ,
    /// stat(2) information
    pub fts_statp  : *const Stat  ,
    /// file name
    pub fts_name   : [c_char;1]   ,
}

/// fts_level: level of root parent
pub const FTS_ROOTPARENTLEVEL: c_short  = -1    ;
/// fts_level: level of root
pub const FTS_ROOTLEVEL      : c_short  = 0     ;
/// fts_info: preorder directory
pub const FTS_D              : c_ushort = 1     ;
/// fts_info: directory that causes cycles
pub const FTS_DC             : c_ushort = 2     ;
/// fts_info: none of the above
pub const FTS_DEFAULT        : c_ushort = 3     ;
/// fts_info: unreadable directory
pub const FTS_DNR            : c_ushort = 4     ;
/// fts_info: dot or dot-dot
pub const FTS_DOT            : c_ushort = 5     ;
/// fts_info: postorder directory
pub const FTS_DP             : c_ushort = 6     ;
/// fts_info: error; errno is set
pub const FTS_ERR            : c_ushort = 7     ;
/// fts_info: regular file
pub const FTS_F              : c_ushort = 8     ;
/// fts_info: initialized only
pub const FTS_INIT           : c_ushort = 9     ;
/// fts_info: stat(2) failed
pub const FTS_NS             : c_ushort = 10    ;
/// fts_info: no stat(2) requested
pub const FTS_NSOK           : c_ushort = 11    ;
/// fts_info: symbolic link
pub const FTS_SL             : c_ushort = 12    ;
/// fts_info: symbolic link without target
pub const FTS_SLNONE         : c_ushort = 13    ;
/// fts_info: whiteout object
pub const FTS_W              : c_ushort = 14    ;
/// fts_flags: don't chdir .. to the parent
pub const FTS_DONTCHDIR      : c_ushort = 0x01  ;
/// fts_flags: followed a symlink to get here
pub const FTS_SYMFOLLOW      : c_ushort = 0x02  ;
/// fts_instr: read node again
pub const FTS_AGAIN          : c_int    = 1     ;
/// fts_instr: follow symbolic link
pub const FTS_FOLLOW         : c_int    = 2     ;
/// fts_instr: no instructions
pub const FTS_NOINSTR        : c_int    = 3     ;
/// fts_instr: discard node
pub const FTS_SKIP           : c_int    = 4     ;
/// fts_open options: follow command line symlinks
pub const FTS_COMFOLLOW      : c_int    = 0x0001;
/// fts_open options: logical walk
pub const FTS_LOGICAL        : c_int    = 0x0002;
/// fts_open options: don't change directories
pub const FTS_NOCHDIR        : c_int    = 0x0004;
/// fts_open options: don't get stat info
pub const FTS_NOSTAT         : c_int    = 0x0008;
/// fts_open options: physical walk
pub const FTS_PHYSICAL       : c_int    = 0x0010;
/// fts_open options: return dot and dot-dot
pub const FTS_SEEDOT         : c_int    = 0x0020;
/// fts_open options: don't cross devices
pub const FTS_XDEV           : c_int    = 0x0040;
/// fts_open options: return whiteout information
pub const FTS_WHITEOUT       : c_int    = 0x0080;
/// fts_open options: valid user option mask
pub const FTS_OPTIONMASK     : c_int    = 0x00ff;
/// fts_open options: (private) child names only
pub const FTS_NAMEONLY       : c_int    = 0x0100;
/// fts_open options: (private) unrecoverable error
pub const FTS_STOP           : c_int    = 0x0200;

extern {
    /// fts_open() in fts.h
    ///
    /// # C function
    /// FTS *fts_open(char * const *path_argv, int options, 
    ///               int (*compar)(const FTSENT **, const FTSENT **));
    ///
    /// # Safety
    /// `path_argv` must be a pointer of a null-terminated array of C strings( null-terminated ).
    ///
    /// `options` must contain either `FTS_LOGICAL` or `FTS_PHYSICAL`.
    ///
    /// # Examples
    /// ```
    /// let path  = std::ffi::CString::new( "." ).unwrap().as_ptr();
    /// let paths = vec![path, std::ptr::null()];
    /// let _fts  = unsafe { fts::ffi::fts_open( paths.as_ptr(), fts::ffi::FTS_LOGICAL, None ) };
    /// ```
    pub fn fts_open( path_argv: *const *const c_char,
                     options  : c_int,
                     compar   : Option<extern "C" fn( *const *const FTSENT, *const *const FTSENT ) -> c_int> ) -> *mut FTS;

    /// fts_read() in fts.h
    ///
    /// # C function
    /// FTSENT *fts_read(FTS *ftsp);
    ///
    /// # Safety
    /// `ftsp` must be a valid pointer of struct FTS.
    ///
    /// # Examples
    /// ```
    /// let path    = std::ffi::CString::new( "." ).unwrap().as_ptr();
    /// let paths   = vec![path, std::ptr::null()];
    /// let fts     = unsafe { fts::ffi::fts_open( paths.as_ptr(), fts::ffi::FTS_LOGICAL, None ) };
    /// let _ftsent = unsafe { fts::ffi::fts_read( fts ) };
    /// ```
    pub fn fts_read( ftsp: *mut FTS ) -> *const FTSENT;

    /// fts_children() in fts.h
    ///
    /// # C function
    /// FTSENT *fts_children(FTS *ftsp, int options);
    ///
    /// # Safety
    /// `ftsp` must be a valid pointer of struct FTS.
    ///
    /// `options` must be either 0 or `FTS_NAMEONLY`.
    ///
    /// # Examples
    /// ```
    /// let path    = std::ffi::CString::new( "." ).unwrap().as_ptr();
    /// let paths   = vec![path, std::ptr::null()];
    /// let fts     = unsafe { fts::ffi::fts_open( paths.as_ptr(), fts::ffi::FTS_LOGICAL, None ) };
    /// let _ftsent = unsafe { fts::ffi::fts_children( fts, 0 ) };
    /// ```
    pub fn fts_children( ftsp: *mut FTS, options: c_int ) -> *const FTSENT;

    /// fts_set() in fts.h
    ///
    /// # C function
    /// int fts_set(FTS *ftsp, FTSENT *f, int options);
    ///
    /// # Safety
    /// `ftsp` must be a valid pointer of struct FTS.
    ///
    /// `f` must be a valid pointer of struct FTSENT.
    ///
    /// `options` must be `FTS_AGAIN`, `FTS_FOLLOW` or `FTS_SKIP`.
    ///
    /// # Examples
    /// ```
    /// let path   = std::ffi::CString::new( "." ).unwrap().as_ptr();
    /// let paths  = vec![path, std::ptr::null()];
    /// let fts    = unsafe { fts::ffi::fts_open( paths.as_ptr(), fts::ffi::FTS_LOGICAL, None ) };
    /// let ftsent = unsafe { fts::ffi::fts_read( fts ) };
    /// let _      = unsafe { fts::ffi::fts_set( fts, ftsent, fts::ffi::FTS_AGAIN ) };
    /// ```
    pub fn fts_set( ftsp: *mut FTS, f: *const FTSENT, options: c_int ) -> c_int;

    /// fts_close() in fts.h
    ///
    /// # C function
    /// int fts_close(FTS *ftsp);
    ///
    /// # Safety
    /// `ftsp` must be a valid pointer of struct FTS.
    ///
    /// # Examples
    /// ```
    /// let path   = std::ffi::CString::new( "." ).unwrap().as_ptr();
    /// let paths  = vec![path, std::ptr::null()];
    /// let fts    = unsafe { fts::ffi::fts_open( paths.as_ptr(), fts::ffi::FTS_LOGICAL, None ) };
    /// let _      = unsafe { fts::ffi::fts_close( fts ) };
    /// ```
    pub fn fts_close( ftsp: *mut FTS ) -> c_int;
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn ftsent_valid( ftsent: *const FTSENT ) {
        unsafe {
            assert!( !ftsent               .is_null() );
            assert!( !(*ftsent).fts_accpath.is_null() );
            assert!( !(*ftsent).fts_path   .is_null() );
            assert!(  (*ftsent).fts_pathlen != 0      );
            assert!(  (*ftsent).fts_namelen != 0      );
            assert!(  (*ftsent).fts_level   >= -1     );
            assert!(  (*ftsent).fts_number  == 0      );
            assert!(  (*ftsent).fts_pointer.is_null() );
            assert!( !(*ftsent).fts_parent .is_null() );
            assert!( !(*ftsent).fts_statp  .is_null() );
        }
    }

    #[test]
    fn normal() {
        unsafe {
            let path  = CString::new( "." ).unwrap().as_ptr();
            let paths = vec![path, ptr::null()];
            let fts   = fts_open( paths.as_ptr(), FTS_LOGICAL, None );
            assert!( !fts.is_null() );

            let mut ftsent = fts_read( fts );
            while !ftsent.is_null() {
                ftsent_valid( ftsent );
                ftsent = fts_read( fts );
            }

            let ret = fts_close( fts );
            assert!( ret == 0 );
        }
    }

    #[test]
    fn children() {
        unsafe {
            let path  = CString::new( "." ).unwrap().as_ptr();
            let paths = vec![path, ptr::null()];
            let fts   = fts_open( paths.as_ptr(), FTS_LOGICAL, None );
            assert!( !fts.is_null() );

            let _ = fts_read( fts );
            let mut ftsent = fts_children( fts, 0 );
            while !ftsent.is_null() {
                ftsent_valid( ftsent );
                ftsent = (*ftsent).fts_link;
            }

            let ret = fts_close( fts );
            assert!( ret == 0 );
        }
    }
}
