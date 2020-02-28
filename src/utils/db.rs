use std::path::Path;
use std::time::Duration;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn get_db(path: &Path) -> PickleDb {
    let db_path = path.join(".db");

    let loaded_result =  PickleDb::load(
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