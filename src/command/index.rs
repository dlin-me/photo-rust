use super::super::utils;
use indicatif::ProgressBar;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

pub fn run(path: &Path, force: bool) {
    let mut db = utils::db::get_db(path);
    let file_count = WalkDir::new(path).into_iter().count();
    let bar = ProgressBar::new(file_count as u64);
    let paths = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());
    let mut all_file_paths = HashSet::new();

    for entry in paths {
        bar.inc(1);
        if entry.file_type().is_file()
            && !entry
                .file_name()
                .to_str()
                .map(|s| s.starts_with("."))
                .unwrap_or(false)
        {
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            let is_in_db = db.get::<String>(path_str).is_some();
            if !is_in_db || force {
                let hash = utils::file::file_md5(path).unwrap();
                db.set(path_str, &hash).unwrap();
            }
            all_file_paths.insert(String::from(path_str));
        }
    }
    bar.finish();

    let mut absent_file_paths = HashSet::new();

    for kv in db.iter() {
        let key = kv.get_key();
        if !all_file_paths.contains(key) {
            absent_file_paths.insert(String::from(key));
        }
    }

    let absent_file_count = absent_file_paths.len();
    if absent_file_count > 0 {
        let bar = ProgressBar::new(absent_file_count as u64);

        for p in absent_file_paths {
            db.rem(&p).unwrap();
            bar.inc(1);
        }
        bar.finish();
    }
}
