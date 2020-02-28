mod command;
mod utils;

use std::path::Path;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("Photo-rust")
        .version("1.0")
        .author("David Lin <davidforest@gmail.com>")
        .about("Photo management CLI tool in Rust")
        .arg(
            Arg::with_name("DIR")
                .long("dir")
                .default_value(".")
                .help("Manage files in the given directory")
                .takes_value(true),
        ).arg(
            Arg::with_name("DRYRUN")
                .long("dryrun")
                .help("Run file indexing in dry mode")
        )
        .subcommand(
            SubCommand::with_name("index").about("Index files"),
        )
        .get_matches();

    let directory = Path::new(matches.value_of("DIR").unwrap());
    let dryrun =  matches.is_present("DRYRUN");
    if let Some(_) = matches.subcommand_matches("index") {
        command::index::run(directory, dryrun);
    }
}
