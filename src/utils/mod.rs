use md5;
use std::io;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

pub fn md5_file(path: &Path) -> io::Result<md5::Digest> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let hash = md5::compute(buffer);

    Ok(hash)
}