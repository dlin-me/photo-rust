use std::io;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use md5;

pub fn file_md5(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let hash = format!("{:x}", md5::compute(buffer));

    Ok(hash)
}