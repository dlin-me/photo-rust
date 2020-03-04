use md5;
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn file_md5(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    let hash = format!("{:x}", md5::compute(buffer));

    Ok(hash)
}

pub fn delete_file(path: &Path) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}
