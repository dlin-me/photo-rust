mod command;
mod utils;

use clap::{App, Arg, SubCommand};
use std::path::Path;

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
        )
        .subcommand(
            SubCommand::with_name("index").about("Index files").arg(
                Arg::with_name("FORCE")
                    .short("f")
                    .long("force")
                    .help("Force re-indexing all files"),
            ),
        )
        .get_matches();

    let directory = Path::new(matches.value_of("DIR").unwrap());
    if let Some(_) = matches.subcommand_matches("index") {
        let force = matches.is_present("FORCE");

        command::index::run(directory, force);
    }
}
