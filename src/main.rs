use clap::{App, Arg};
use directories::UserDirs;
use std::fs;
use chrono;
use chrono::Datelike;
use std::process::Command;
use std::env;
use std::env::var;

fn read_journal(date: String) {
    // SHOW JOURNAL NOTE
    if date == "" {
        let date = chrono::Utc::now();
        let user_dirs = UserDirs::new();
        let user_dirs = user_dirs.unwrap();
        let docs = user_dirs.document_dir().unwrap();

        let path = format!("{}/Journal/{}/{}/{}/JOURNAL-{}-{}-{}.md", docs.to_str().unwrap(), date.year(), date.month(), date.day(), date.year(), date.month(), date.day());

        let exists = String::from_utf8(Command::new("which")
            .arg("glow")
            .output().unwrap().stdout).unwrap();

        if (exists != "") { // SHOW NOTE WITH GLOW
            Command::new("glow")
                .arg(&path)
                .status().unwrap();
        } else { // FALL BACK TO PLAIN TEXT
            let f = fs::read_to_string(&path).unwrap();
            println!("{}", f);
        }
    }
}

fn list_journals() {
    // DATE FORMATTING / LIST WITH FILTERS
}

fn write_journal() {
    // OPEN JOURNAL FILE WITH EDITOR
    let date = chrono::Utc::now();

    let user_dirs = UserDirs::new();
    let user_dirs = user_dirs.unwrap();
    let docs = user_dirs.document_dir().unwrap();
    fs::create_dir_all(format!("{}/Journal/{}/{}/{}", docs.to_str().unwrap(), date.year(), date.month(), date.day()));

    Command::new(var("EDITOR").ok().unwrap_or_else(|| "nano".to_string()))
        .arg(format!("{}/Journal/{}/{}/{}/JOURNAL-{}-{}-{}.md", docs.to_str().unwrap(), date.year(), date.month(), date.day(), date.year(), date.month(), date.day()))
        .status().unwrap();
}

fn ensure_dirs() {
    let user_dirs = UserDirs::new();
    let user_dirs = user_dirs.unwrap();
    let docs = user_dirs.document_dir().unwrap();
    fs::create_dir_all(format!("{}/Journal", docs.to_str().unwrap()));
}

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
            App::new("read")
                .about("read entry")
                .arg(
                    Arg::with_name("date")
                        .required(false)
                        .default_value("")
                        .help("Journal Date"),
                )
        )
        .get_matches();

    ensure_dirs();

    match args.subcommand() {
        ("list", Some(cmd)) => {
            list_journals();
        }
        ("read", Some(cmd)) => {
            read_journal(cmd.value_of("date").unwrap().to_string());
        }
        ("", None) => {
            write_journal();
        }
        _ => {
            println!("{}", args.usage());
        }
    }

}
