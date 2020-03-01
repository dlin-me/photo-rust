use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

use super::time::unix_secs;

#[derive(Serialize, Deserialize)]
pub struct Hash {
    md5: String,
    time: u64,
}

impl Hash {
    pub fn new(md5: String) -> Hash {
        let time = unix_secs();
        Hash { md5, time }
    }
}

pub fn get_db(path: &Path) -> PickleDb {
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
