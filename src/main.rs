mod etc;
mod list;
mod read;
mod search;
mod write;

use clap::{App, Arg};
use std::ops::Add;

fn main() {
    let args = App::new("Journal")
        .version(option_env!("CARGO_PKG_VERSION").unwrap())
        .author("JMARyA <jmarya0@icloud.com>")
        .about("Journaling App")
        .subcommand(
            App::new("list")
                .about("list entries")
                .arg(
                    Arg::with_name("year")
                        .short("y")
                        .long("year")
                        .required(false)
                        .value_name("YEAR")
                        .help("Year"),
                )
                .arg(
                    Arg::with_name("month")
                        .short("m")
                        .long("month")
                        .help("month"),
                ),
        )
        .subcommand(
            App::new("read").about("read entry").arg(
                Arg::with_name("date")
                    .required(false)
                    .default_value("")
                    .help("Journal Date"),
            ),
        )
        .subcommand(
            App::new("search").about("search through entries").arg(
                Arg::with_name("pattern")
                    .required(false)
                    .default_value("")
                    .help("Search Pattern")
                    .multiple(true),
            ),
        )
        .subcommand(
            App::new("write")
                .about("write a note")
                .arg(Arg::with_name("name").required(true).help("Name of Note")),
        )
        .get_matches();
    etc::ensure_dirs();

    match args.subcommand() {
        ("list", Some(cmd)) => {
            let _ = cmd; // USE LATER
            list::list_journals();
        }
        ("read", Some(cmd)) => {
            read::read_journal(cmd.value_of("date").unwrap().to_string());
        }
        ("write", Some(cmd)) => {
            write::write_journal(Some(cmd.value_of("name").unwrap()));
        }
        ("search", Some(cmd)) => {
            // Prepare search pattern
            let patterns = cmd.values_of("pattern").unwrap();
            let mut pattern = String::new();
            for i in patterns.enumerate() {
                pattern = pattern.add(i.1);
                pattern = pattern.add(" ");
            }
            pattern.truncate(pattern.len() - 1);

            search::search_journal(pattern);
        }
        ("", None) => {
            write::write_journal(None);
        }
        _ => {
            println!("{}", args.usage());
        }
    }
}
