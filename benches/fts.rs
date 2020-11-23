#![feature(test)]

pub extern crate fts;
pub extern crate test;
pub extern crate walkdir;

use std::fs;
use std::path::PathBuf;
use test::Bencher;

// ---------------------------------------------------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------------------------------------------------

fn readdir_cnt(base: PathBuf, cnt: isize) -> isize {
    let reader = match fs::read_dir(&base) {
        Ok(x) => x,
        Err(_) => return cnt,
    };

    let mut tmp = cnt;
    for entry in reader {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            tmp = tmp + 1;
        } else if file_type.is_symlink() {
        } else {
            tmp = readdir_cnt(entry.path(), tmp)
        }
    }
    tmp
}

fn readdir_cnt_size(base: PathBuf, cnt: isize, size: isize) -> (isize, isize) {
    let reader = match fs::read_dir(&base) {
        Ok(x) => x,
        Err(_) => return (cnt, size),
    };

    let mut tmp0 = cnt;
    let mut tmp1 = size;
    for entry in reader {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            tmp0 = tmp0 + 1;
            tmp1 = tmp1 + entry.metadata().unwrap().len() as isize;
        } else if file_type.is_symlink() {
        } else {
            let (t0, t1) = readdir_cnt_size(entry.path(), tmp0, tmp1);
            tmp0 = t0;
            tmp1 = t1;
        }
    }
    (tmp0, tmp1)
}

// ---------------------------------------------------------------------------------------------------------------------
// File Count
// ---------------------------------------------------------------------------------------------------------------------

#[bench]
fn fts_walkdir(b: &mut Bencher) {
    b.iter(|| {
        let mut _cnt = 0;
        for entry in
            fts::walkdir::WalkDir::new(fts::walkdir::WalkDirConf::new("/usr").no_metadata())
        {
            match entry {
                Ok(x) => if x.file_type().is_file() {
                    _cnt += 1
                },
                Err(_) => (),
            }
        }
        //println!( "{} {}", _cnt );
    });
}

#[bench]
fn walkdir(b: &mut Bencher) {
    b.iter(|| {
        let mut _cnt = 0;
        for entry in walkdir::WalkDir::new("/usr") {
            match entry {
                Ok(x) => if x.file_type().is_file() {
                    _cnt += 1
                },
                Err(_) => (),
            }
        }
        //println!( "{} {}", _cnt );
    });
}

#[bench]
fn readdir(b: &mut Bencher) {
    b.iter(|| {
        let _ret = readdir_cnt(PathBuf::from("/usr"), 0);
        //println!( "{}", _ret );
    });
}

// ---------------------------------------------------------------------------------------------------------------------
// Total Bytes
// ---------------------------------------------------------------------------------------------------------------------

#[bench]
fn fts_walkdir_metadata(b: &mut Bencher) {
    b.iter(|| {
        let mut _cnt = 0;
        let mut _size = 0;
        for entry in fts::walkdir::WalkDir::new(fts::walkdir::WalkDirConf::new("/usr")) {
            match entry {
                Ok(x) => if x.file_type().is_file() {
                    _cnt += 1;
                    _size += x.metadata().unwrap().len()
                },
                Err(_) => (),
            }
        }
        //println!( "{} {}", _cnt, _size );
    });
}

#[bench]
fn walkdir_metadata(b: &mut Bencher) {
    b.iter(|| {
        let mut _cnt = 0;
        let mut _size = 0;
        for entry in walkdir::WalkDir::new("/usr") {
            match entry {
                Ok(x) => if x.file_type().is_file() {
                    _cnt += 1;
                    _size += x.metadata().unwrap().len()
                },
                Err(_) => (),
            }
        }
        //println!( "{} {}", _cnt, _size );
    });
}

#[bench]
fn readdir_metadata(b: &mut Bencher) {
    b.iter(|| {
        let (_ret0, _ret1) = readdir_cnt_size(PathBuf::from("/usr"), 0, 0);
        //println!( "{} {}", _ret0, _ret1 );
    });
}
