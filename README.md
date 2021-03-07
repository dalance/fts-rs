# fts-rs

[![Actions Status](https://github.com/dalance/fts-rs/workflows/Regression/badge.svg)](https://github.com/dalance/fts-rs/actions)
[![Crates.io](https://img.shields.io/crates/v/fts.svg)](https://crates.io/crates/fts)
[![Docs.rs](https://docs.rs/fts/badge.svg)](https://docs.rs/fts)
[![codecov](https://codecov.io/gh/dalance/fts-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/dalance/fts-rs)

A Rust library for high performance directory walking using libc fts.

[Documentation](https://docs.rs/fts)

## Usage

```Cargo.toml
[dependencies]
fts = "0.3.0"
```

## Example

Use `WalkDir` for directory walking:

```rust,skt-default
use std::path::Path;
use fts::walkdir::{WalkDir, WalkDirConf};

let path = Path::new( "." );
for p in WalkDir::new( WalkDirConf::new( path ) ) {
    println!( "{:?}", p.unwrap() );
}
```

Call `fts_*` function directly:

```rust,skt-default
use std::ffi::CString;
use fts::ffi::{fts_open, fts_read, fts_close, FTS_LOGICAL};

let path    = CString::new( "." ).unwrap();
let paths   = vec![path.as_ptr(), std::ptr::null()];
let fts     = unsafe { fts_open ( paths.as_ptr(), FTS_LOGICAL, None ) };
let _ftsent = unsafe { fts_read ( fts ) };
let _       = unsafe { fts_close( fts ) };
```

## Benchmark

A `cargo bench` result is the following.
`fts_walkdir` is this library, `readdir` is `std::fs:read_dir`, `walkdir` is [walkdir::WalkDir](https://github.com/BurntSushi/walkdir).
a suffix `_metadata` means using call `DirEntry::metadata()`.

```
test fts_walkdir          ... bench: 315,114,126 ns/iter (+/- 8,478,709)
test fts_walkdir_metadata ... bench: 480,089,245 ns/iter (+/- 11,478,335)
test readdir              ... bench: 575,856,224 ns/iter (+/- 15,021,486)
test readdir_metadata     ... bench: 790,838,218 ns/iter (+/- 12,780,010)
test walkdir              ... bench: 688,884,058 ns/iter (+/- 8,023,838)
test walkdir_metadata     ... bench: 904,379,691 ns/iter (+/- 10,212,776)
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
