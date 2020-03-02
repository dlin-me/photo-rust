use super::super::utils;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn run() {
    let db = utils::db::get_db();
    
    let mut dup_map: HashMap<String, Vec<(String, u64)>> = HashMap::new();

    for kv in db.iter() {
        let key:String = kv.get_key().to_string();
        let value:utils::db::FileHash = kv.get_value().unwrap();
        let new_value = (key, value.time);

        match dup_map.entry(value.md5) {
            Entry::Vacant(e) => { e.insert(vec![new_value]); },
            Entry::Occupied(mut e) => { e.get_mut().push(new_value); }
        }
    }

    dup_map.retain(|_, v| v.len() > 1);
 
    println!("{:?}", dup_map);
}
