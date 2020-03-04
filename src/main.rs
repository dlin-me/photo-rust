mod command;
mod utils;

use clap::{App, Arg, SubCommand};


fn main() {
    let path = "/Users/dlin/Desktop/emoj";
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
            )
        )
        .subcommand(
            SubCommand::with_name("dups").about("List duplicated files")
            .arg(
                Arg::with_name("DELETE")
                    .short("d")
                    .long("delete")
                    .help("Delete duplicate files"),                    
            ).arg(
                Arg::with_name("INTERACTIVE")
                    .short("i")
                    .long("interactive")
                    .help("Delete duplicate files interactively"),                    
            )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("index") {
        let force = matches.is_present("FORCE");
        command::index::run(path, force);
    } else if let Some(matches) = matches.subcommand_matches("dups") {

        let delete = matches.is_present("DELETE");
        let interative = matches.is_present("INTERACTIVE");

        command::dups::run(path, delete, interative);
    } 
}
