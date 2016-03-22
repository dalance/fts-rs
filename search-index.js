var searchIndex = {};
searchIndex['fts'] = {"items":[[0,"ffi","fts","Bindings for libc fts.",null,null],[3,"FTSENT","fts::ffi","struct FTSENT in fts.h",null,null],[12,"fts_cycle","","cycle node",0,null],[12,"fts_parent","","parent directory",0,null],[12,"fts_link","","next file in directory",0,null],[12,"fts_number","","local numeric value",0,null],[12,"fts_pointer","","local address value",0,null],[12,"fts_accpath","","access path",0,null],[12,"fts_path","","root path",0,null],[12,"fts_errno","","errno for this node",0,null],[12,"fts_symfd","","fd for symlink",0,null],[12,"fts_pathlen","","strlen(fts_path)",0,null],[12,"fts_namelen","","strlen(fts_name)",0,null],[12,"fts_ino","","inode",0,null],[12,"fts_dev","","device",0,null],[12,"fts_nlink","","link count",0,null],[12,"fts_level","","depth (-1 to N)",0,null],[12,"fts_info","","user flags for FTSENT structure",0,null],[12,"fts_flags","","private flags for FTSENT structure",0,null],[12,"fts_instr","","fts_set() instructions",0,null],[12,"fts_statp","","stat(2) information",0,null],[12,"fts_name","","file name",0,null],[4,"FTS","","struct FTS in fts.h ( opaque struct )",null,null],[5,"fts_open","","fts_open() in fts.h",null,null],[5,"fts_read","","fts_read() in fts.h",null,null],[5,"fts_children","","fts_children() in fts.h",null,null],[5,"fts_set","","fts_set() in fts.h",null,null],[5,"fts_close","","fts_close() in fts.h",null,null],[17,"FTS_ROOTPARENTLEVEL","","fts_level: level of root parent",null,null],[17,"FTS_ROOTLEVEL","","fts_level: level of root",null,null],[17,"FTS_D","","fts_info: preorder directory",null,null],[17,"FTS_DC","","fts_info: directory that causes cycles",null,null],[17,"FTS_DEFAULT","","fts_info: none of the above",null,null],[17,"FTS_DNR","","fts_info: unreadable directory",null,null],[17,"FTS_DOT","","fts_info: dot or dot-dot",null,null],[17,"FTS_DP","","fts_info: postorder directory",null,null],[17,"FTS_ERR","","fts_info: error; errno is set",null,null],[17,"FTS_F","","fts_info: regular file",null,null],[17,"FTS_INIT","","fts_info: initialized only",null,null],[17,"FTS_NS","","fts_info: stat(2) failed",null,null],[17,"FTS_NSOK","","fts_info: no stat(2) requested",null,null],[17,"FTS_SL","","fts_info: symbolic link",null,null],[17,"FTS_SLNONE","","fts_info: symbolic link without target",null,null],[17,"FTS_W","","fts_info: whiteout object",null,null],[17,"FTS_DONTCHDIR","","fts_flags: don't chdir .. to the parent",null,null],[17,"FTS_SYMFOLLOW","","fts_flags: followed a symlink to get here",null,null],[17,"FTS_AGAIN","","fts_instr: read node again",null,null],[17,"FTS_FOLLOW","","fts_instr: follow symbolic link",null,null],[17,"FTS_NOINSTR","","fts_instr: no instructions",null,null],[17,"FTS_SKIP","","fts_instr: discard node",null,null],[17,"FTS_COMFOLLOW","","fts_open options: follow command line symlinks",null,null],[17,"FTS_LOGICAL","","fts_open options: logical walk",null,null],[17,"FTS_NOCHDIR","","fts_open options: don't change directories",null,null],[17,"FTS_NOSTAT","","fts_open options: don't get stat info",null,null],[17,"FTS_PHYSICAL","","fts_open options: physical walk",null,null],[17,"FTS_SEEDOT","","fts_open options: return dot and dot-dot",null,null],[17,"FTS_XDEV","","fts_open options: don't cross devices",null,null],[17,"FTS_WHITEOUT","","fts_open options: return whiteout information",null,null],[17,"FTS_OPTIONMASK","","fts_open options: valid user option mask",null,null],[17,"FTS_NAMEONLY","","fts_open options: (private) child names only",null,null],[17,"FTS_STOP","","fts_open options: (private) unrecoverable error",null,null],[11,"fmt","","",0,{"inputs":[{"name":"ftsent"},{"name":"formatter"}],"output":{"name":"result"}}],[0,"fts","fts","A wrapper library of libc fts.",null,null],[3,"FtsEntry","fts::fts","",null,null],[12,"path","","",1,null],[12,"name","","",1,null],[12,"info","","",1,null],[12,"stat","","",1,null],[12,"level","","",1,null],[12,"error","","",1,null],[3,"Fts","","",null,null],[3,"FtsComp","","",null,null],[4,"FtsSetOption","","",null,null],[13,"Again","","read node again",2,null],[13,"Follow","","follow symbolic link",2,null],[13,"Skip","","discard node",2,null],[4,"FtsInfo","","",null,null],[13,"IsDir","","",3,null],[13,"IsDirCyclic","","",3,null],[13,"IsDefault","","",3,null],[13,"IsDontRead","","",3,null],[13,"IsDot","","",3,null],[13,"IsDirPost","","",3,null],[13,"IsErr","","",3,null],[13,"IsFile","","",3,null],[13,"IsNoStat","","",3,null],[13,"IsNoStatOk","","",3,null],[13,"IsSymlink","","",3,null],[13,"IsSymlinkNone","","",3,null],[13,"IsUnknown","","",3,null],[4,"FtsError","","",null,null],[13,"PathWithNull","","path string contains null charactors.",4,null],[13,"SetFail","","fts_set() failed.",4,null],[0,"fts_option","","",null,null],[3,"Flags","fts::fts::fts_option","",null,null],[17,"COMFOLLOW","","follow command line symlinks",null,null],[17,"LOGICAL","","logical walk",null,null],[17,"NOCHDIR","","don't change directories",null,null],[17,"NOSTAT","","don't get stat info",null,null],[17,"PHYSICAL","","physical walk",null,null],[17,"SEEDOT","","return dot and dot-dot",null,null],[17,"XDEV","","don't cross devices",null,null],[11,"hash","","",5,null],[11,"cmp","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"option"}}],[11,"lt","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"le","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"gt","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"ge","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"clone","","",5,{"inputs":[{"name":"flags"}],"output":{"name":"flags"}}],[11,"eq","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"fmt","","",5,{"inputs":[{"name":"flags"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"empty","","Returns an empty set of flags.",5,{"inputs":[{"name":"flags"}],"output":{"name":"flags"}}],[11,"all","","Returns the set containing all flags.",5,{"inputs":[{"name":"flags"}],"output":{"name":"flags"}}],[11,"bits","","Returns the raw value of the flags currently stored.",5,{"inputs":[{"name":"flags"}],"output":{"name":"u32"}}],[11,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",5,{"inputs":[{"name":"flags"},{"name":"u32"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",5,{"inputs":[{"name":"flags"},{"name":"u32"}],"output":{"name":"flags"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",5,{"inputs":[{"name":"flags"}],"output":{"name":"bool"}}],[11,"is_all","","Returns `true` if all flags are currently set.",5,{"inputs":[{"name":"flags"}],"output":{"name":"bool"}}],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"bool"}}],[11,"insert","","Inserts the specified flags in-place.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":null}],[11,"remove","","Removes the specified flags in-place.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":null}],[11,"toggle","","Toggles the specified flags in-place.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":null}],[11,"bitor","","Returns the union of the two sets of flags.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"bitand","","Returns the intersection between the two sets of flags.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"sub","","Returns the set difference of the two sets of flags.",5,{"inputs":[{"name":"flags"},{"name":"flags"}],"output":{"name":"flags"}}],[11,"not","","Returns the complement of this set of flags.",5,{"inputs":[{"name":"flags"}],"output":{"name":"flags"}}],[11,"from_iter","","",5,{"inputs":[{"name":"flags"},{"name":"t"}],"output":{"name":"flags"}}],[6,"FtsCompIn","fts::fts","",null,null],[6,"FtsCompFunc","","",null,null],[11,"fmt","","",2,{"inputs":[{"name":"ftssetoption"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",3,{"inputs":[{"name":"ftsinfo"},{"name":"ftsinfo"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"ftsinfo"},{"name":"ftsinfo"}],"output":{"name":"bool"}}],[11,"fmt","","",3,{"inputs":[{"name":"ftsinfo"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",3,{"inputs":[{"name":"ftsinfo"}],"output":{"name":"ftsinfo"}}],[11,"from_i64","","",3,{"inputs":[{"name":"ftsinfo"},{"name":"i64"}],"output":{"name":"option"}}],[11,"from_u64","","",3,{"inputs":[{"name":"ftsinfo"},{"name":"u64"}],"output":{"name":"option"}}],[11,"fmt","","",4,{"inputs":[{"name":"ftserror"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",1,{"inputs":[{"name":"ftsentry"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",6,{"inputs":[{"name":"fts"},{"name":"vec"},{"name":"flags"},{"name":"option"}],"output":{"name":"result"}}],[11,"read","","",6,{"inputs":[{"name":"fts"}],"output":{"name":"option"}}],[11,"set","","",6,{"inputs":[{"name":"fts"},{"name":"ftsentry"},{"name":"ftssetoption"}],"output":{"name":"result"}}],[11,"drop","","",6,{"inputs":[{"name":"fts"}],"output":null}],[11,"by_name_ascending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_name_descending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_atime_ascending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_atime_descending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_mtime_ascending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_mtime_descending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_ctime_ascending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_ctime_descending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_len_ascending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[11,"by_len_descending","","",7,{"inputs":[{"name":"ftscomp"},{"name":"ftscompin"},{"name":"ftscompin"}],"output":{"name":"c_int"}}],[0,"walkdir","fts","A library for directory walking.",null,null],[3,"DirEntry","fts::walkdir","A directory entry like `std::fs::DirEntry`.",null,null],[3,"FileType","","A file type of the directory entry like `std::fs::FileType`.",null,null],[3,"Iter","","A iterator for enumerating directory entries.",null,null],[3,"WalkDirConf","","A configuration builder of the settings for directory walking.",null,null],[3,"WalkDir","","A builder to create an iterator for directory walking.",null,null],[11,"path","","Returns the full path to the file that this entry represents.",8,{"inputs":[{"name":"direntry"}],"output":{"name":"path"}}],[11,"metadata","","Return the metadata for the file that this entry points at.",8,{"inputs":[{"name":"direntry"}],"output":{"name":"option"}}],[11,"file_type","","Return the file type for the file that this entry points at.",8,{"inputs":[{"name":"direntry"}],"output":{"name":"filetype"}}],[11,"file_name","","Returns the bare file name of this directory entry without any other leading path component.",8,{"inputs":[{"name":"direntry"}],"output":{"name":"osstr"}}],[11,"depth","","Returns the depth at which this entry was created relative to the original path to `WalkDir::new`.",8,{"inputs":[{"name":"direntry"}],"output":{"name":"usize"}}],[11,"fmt","","",8,{"inputs":[{"name":"direntry"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"is_dir","","Test whether this file type represents a directory.",9,{"inputs":[{"name":"filetype"}],"output":{"name":"bool"}}],[11,"is_file","","Test whether this file type represents a regular file.",9,{"inputs":[{"name":"filetype"}],"output":{"name":"bool"}}],[11,"is_symlink","","Test whether this file type represents a symbolic link.",9,{"inputs":[{"name":"filetype"}],"output":{"name":"bool"}}],[11,"next","","",10,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[11,"new","","Create new `WalkDirConf` with the root directory for directory walking.",11,{"inputs":[{"name":"walkdirconf"},{"name":"p"}],"output":{"name":"self"}}],[11,"follow_symlink","","Enable following symblic links.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"cross_device","","Enable following symblic links across devices .",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"include_dot","","Enable enumerating `.` and `..`.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"no_metadata","","Disable providing metadata.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"no_chdir","","Disable changing current directory through directory walking.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"sort_by_name","","Sort by file name.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"sort_by_len","","Sort by file length.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"sort_by_atime","","Sort by access time.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"sort_by_ctime","","Sort by create time.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"sort_by_mtime","","Sort by modify time.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"sort_ascending","","Sort by ascending order.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"sort_descending","","Sort by descending order.",11,{"inputs":[{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"new","","Create new `WalkDir` configured by specified `WalkDirConf`.",12,{"inputs":[{"name":"walkdir"},{"name":"walkdirconf"}],"output":{"name":"self"}}],[11,"path","","Return the base directory for directory walking.",12,{"inputs":[{"name":"walkdir"}],"output":{"name":"str"}}],[11,"is_follow_symlink","","Test whether `WalkDir` follows symblic links.",12,{"inputs":[{"name":"walkdir"}],"output":{"name":"bool"}}],[11,"is_cross_device","","Test whether `WalkDir` follows symblic links across devices.",12,{"inputs":[{"name":"walkdir"}],"output":{"name":"bool"}}],[11,"is_include_dot","","Test whether `WalkDir` enumerates `.` and `..`.",12,{"inputs":[{"name":"walkdir"}],"output":{"name":"bool"}}],[11,"is_no_metadata","","Test whether `WalkDir` provides metadata.",12,{"inputs":[{"name":"walkdir"}],"output":{"name":"bool"}}],[11,"is_no_chdir","","Test whether `WalkDir` change current directory through directory walking.",12,{"inputs":[{"name":"walkdir"}],"output":{"name":"bool"}}],[11,"into_iter","","",12,{"inputs":[{"name":"walkdir"}],"output":{"name":"iter"}}]],"paths":[[3,"FTSENT"],[3,"FtsEntry"],[4,"FtsSetOption"],[4,"FtsInfo"],[4,"FtsError"],[3,"Flags"],[3,"Fts"],[3,"FtsComp"],[3,"DirEntry"],[3,"FileType"],[3,"Iter"],[3,"WalkDirConf"],[3,"WalkDir"]]};
initSearch(searchIndex);
