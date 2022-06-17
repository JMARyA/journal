use crate::etc;
use std::fs;
use std::process::Command;

pub fn read_journal(name: String) {
    // SHOW JOURNAL NOTE
    let mut path = String::new();

    if name == "" {
        // SHOW TODAYS ENTRY
        path = etc::today_journal_path();
    }
    // Show Journal Entry with Date
    if path == "" {
        // Show Note
        path = format!("{}/Notes/{}.md", etc::journal_dir(), name);
    }

    // If found show with cli tools
    if path != "" {
        if etc::binary_exists("glow") {
            // SHOW NOTE WITH GLOW
            Command::new("glow").arg(&path).status().unwrap();
        } else {
            // FALL BACK TO PLAIN TEXT
            let f = fs::read_to_string(&path).unwrap();
            println!("{}", f);
        }
    }
}
