use super::super::utils;
use pickledb::PickleDb;
use colored::*;
use read_input::prelude::*;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::path::Path;

pub fn run(target_path: &str, delete: bool, interactive: bool) {
    let mut db = utils::db::get_db(target_path);
    let mut dup_map: HashMap<String, Vec<(String, u64)>> = HashMap::new();

    for kv in db.iter() {
        let key: String = kv.get_key().to_string();
        let value: utils::db::FileHash = kv.get_value().unwrap();
        let new_value = (key, value.time);

        match dup_map.entry(value.md5) {
            Entry::Vacant(e) => {
                e.insert(vec![new_value]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(new_value);
            }
        }
    }

    dup_map.retain(|_, v| v.len() > 1);

    println!(
        "\n{}\n",
        format!("{} files with duplicates found.", dup_map.len())
            .blue()
            .bold()
    );

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
        let dups_to_remove = dups
            .into_iter()
            .filter(|(p, _)| p != first_path)
            .collect::<Vec<_>>();

        println!(
            "{}",
            format!(
                "{} ({})",
                utils::string::shorten(first_path, 80),
                dups_to_remove.len()
            )
            .green()
            .bold()
        );
        for (path, _) in &dups_to_remove {
            println!("\t{}", utils::string::shorten(path, 80).bright_black());
        }

        if delete {
            delete_dups(&dups_to_remove, &mut db, interactive);
        }
        println!("");
    }

    if dup_map.len() > 10 {
        println!(
            "\n{}\n",
            format!("{} files with duplicates found.", dup_map.len())
                .blue()
                .bold()
        );
    }
}

fn delete_dups(dups: &Vec<&(String, u64)>, db: &mut PickleDb, interactive: bool) -> usize {
    let delete = match interactive {
        true => {
            let user_input: String = input()
                .msg("Delete ? y|n (default n)".blue())
                .err("Please type 'y' or 'n' or ctrl+c to quit.".blue())
                .inside(vec![String::from("y"), String::from("n")])
                .default(String::from("n"))
                .get();
            user_input == "y"
        }
        false => true,
    };
    let mut deleted_counter = 0;

    if delete {
        for (path, _) in dups {
            deleted_counter += match utils::file::delete_file(Path::new(path)) {
                Ok(_) => {
                    db.rem(&path).unwrap();
                    1
                },
                Err(_) => 0,
            }
        }
        println!("{}", format!("Deleted {} file(s).", deleted_counter).red());
    }


    deleted_counter
}
