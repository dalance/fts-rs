# fts-rs

[![Build Status](https://travis-ci.org/dalance/fts-rs.svg?branch=master)](https://travis-ci.org/dalance/fts-rs)

Bindings for libc fts, and a library for high performance directory walking.

[Documentation](http://dalance.github.io/fts-rs/fts/)

## Usage

```Cargo.toml
[dependencies]
fts-rs = "*"
```

## Example

Call fts_* function directly:

```rust,no_run
use std::ffi::CString;
use fts::ffi::{fts_open, fts_read, fts_close, FTS_LOGICAL};

let path    = CString::new( "." ).unwrap().as_ptr();
let paths   = vec![path, std::ptr::null()];
let fts     = unsafe { fts_open ( paths.as_ptr(), FTS_LOGICAL, None ) };
let _ftsent = unsafe { fts_read ( fts ) };
let _       = unsafe { fts_close( fts ) };
```

Use `WalkDir` for directory walking:

```rust,no_run
use std::path::Path;
use fts::walkdir::{WalkDir, WalkDirConf}

let path = Path::new( "." );
for p in WalkDir::new( WalkDirConf::new( path ) ) {
    println!( "{:?}", p.unwrap() );
}
```

## Benchmark

A `cargo bench` result is the following.
`fts_walkdir` is this library, `readdir` is `std::fs:read_dir`, `walkdir` is [walkdir::WaikDir](https://github.com/BurntSushi/walkdir).
a suffix `_metadata` means using call `DirEntry::metadata()`.

```
test fts_walkdir          ... bench: 315,114,126 ns/iter (+/- 8,478,709)
test fts_walkdir_metadata ... bench: 480,089,245 ns/iter (+/- 11,478,335)
test readdir              ... bench: 575,856,224 ns/iter (+/- 15,021,486)
test readdir_metadata     ... bench: 790,838,218 ns/iter (+/- 12,780,010)
test walkdir              ... bench: 688,884,058 ns/iter (+/- 8,023,838)
test walkdir_metadata     ... bench: 904,379,691 ns/iter (+/- 10,212,776)
```
