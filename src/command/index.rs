use super::super::utils;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(path: &Path, _dryrun: bool) {
    let file_count = WalkDir::new(path).into_iter().count();
    println!("{:?}", file_count);
    let paths = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());


    for entry in paths {
        if !entry.file_type().is_dir() {
            println!("{:?}", utils::md5_file(entry.path()));
        }
    }
}
