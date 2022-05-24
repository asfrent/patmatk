use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct RawFiles {
    original: Vec<u8>,
    modified: Vec<u8>,
    target: Vec<u8>,
}

fn read_raw_files(original_path: &str, modified_path: &str, target_path: &str)
    -> io::Result<RawFiles> {
    let mut rf = RawFiles {
        original: Vec::new(),
        modified: Vec::new(),
        target: Vec::new(),
    };
    File::open(original_path)?.read_to_end(&mut rf.original)?;
    File::open(modified_path)?.read_to_end(&mut rf.modified)?;
    File::open(target_path)?.read_to_end(&mut rf.target)?;
    Ok(rf)
}

fn main() -> io::Result<()> {
    let rf = read_raw_files("original.bin", "modified.bin", "target.bin")?;
    Ok(())
}
