use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

use super::time::unix_secs;

#[derive(Serialize, Deserialize)]
pub struct FileHash {
    pub md5: String,
    pub time: u64,
}

impl FileHash {
    pub fn new(md5: String) -> FileHash {
        let time = unix_secs();
        FileHash { md5, time }
    }
}

pub fn get_db() -> PickleDb {
    let path = Path::new(".");
    let db_path = path.join(".photo_rust_db");

    let loaded_result = PickleDb::load(
        &db_path,
        PickleDbDumpPolicy::PeriodicDump(Duration::new(10, 0)),
        SerializationMethod::Bin,
    );

    if let Ok(db) = loaded_result {
        db
    } else {
        PickleDb::new(
            &db_path,
            PickleDbDumpPolicy::PeriodicDump(Duration::new(10, 0)),
            SerializationMethod::Bin,
        )
    }
}
