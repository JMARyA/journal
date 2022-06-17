use crate::etc;
use std::ops::Add;
use std::process::Command;

pub fn search_journal(pattern: String) {
    let path = etc::journal_dir();

    // Build Regex Or Pattern with Words
    let mut parg = String::from("(");
    let pattern_parts: Vec<&str> = pattern.split(" ").collect();
    for word in pattern_parts.iter().enumerate() {
        parg = parg.add(&word.1.to_string());
        if word.0 != pattern_parts.len() - 1 {
            parg = parg.add("|");
        }
    }
    parg = parg.add(")");

    // Search with cli tools
    if etc::binary_exists("rg") {
        Command::new("rg")
            .arg("-i")
            .arg(&parg)
            .arg(&path)
            .status()
            .unwrap();
    } else {
        Command::new("grep")
            .arg("--recursive")
            .arg("-i")
            .arg("-E")
            .arg(&parg)
            .arg(&path)
            .status()
            .unwrap();
    }
}
