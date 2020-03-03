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

    for (_, dups) in &dup_map {
        let mut first = None;
        for (path, time) in dups {
            first = match first {
                None => Some((path, time)),
                Some((_, t)) if t > time => Some((path, time)),
                Some(_) => first,
            };
        }
        let (first_path, _) = first.unwrap();
        println!("{} ({})", shorten(first_path, 80), dups.len());

        for (path, _) in dups {
            println!("\t{}", shorten(path, 80));
        }
        println!("");
    }
}


fn shorten(input: &String, max_len: usize) -> String{
    if input.len() > max_len {
        let diff = input.len() - max_len;
        let to_remove = match diff {
            x if x > 3 => x,
            x if x <= 0 => 0,
            _ => 3
        };

        let head = (input.len() - to_remove) / 2;
        let tail = input.len()  - head - 3;
        let res = format!("{}...{}", &input[..head], &input[tail..]);

        return res;
    }

    return String::from(input);
}