mod command;
mod utils;

use clap::{App, Arg, SubCommand};


fn main() {
    let matches = App::new("Photo-rust")
        .version("1.0")
        .author("David Lin <davidforest@gmail.com>")
        .about("Photo management CLI tool in Rust")
        .subcommand(
            SubCommand::with_name("index").about("Index files").arg(
                Arg::with_name("FORCE")
                    .short("f")
                    .long("force")
                    .help("Force re-indexing all files"),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("index") {
        let force = matches.is_present("FORCE");
        command::index::run(force);
    }
}
