use fts::{Fts, FtsError, FtsEntry, FtsInfo, fts_option};

pub struct FtsExt;

impl FtsExt {
    pub fn read_dir( paths: Vec<String> ) -> Result<FtsIter, FtsError> {
        match Fts::new( paths, fts_option::PHYSICAL ) {
            Ok ( x ) => Ok( FtsIter{ fts: x } ),
            Err( x ) => Err( x ),
        }
    }
}

pub struct FtsIter {
    fts: Fts,
}

impl Iterator for FtsIter {
    type Item = FtsEntry;

    fn next( &mut self ) -> Option<FtsEntry> {
        self.fts.read()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal() {
        let paths = vec![String::from( "." )];
        let iter  = FtsExt::read_dir( paths ).unwrap();
        for _p in iter {
            //println!( "{:?}", p );
        }
    }
}
