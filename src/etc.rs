use chrono::Datelike;
use directories::UserDirs;
use std::fs;
use std::process::Command;
use std::env::var;
use std::path::Path;

pub fn get_document_path() -> String {
    // Check for environment variable
    // ROOT PATH -> $JOURNAL_DIR/Journal
    let doc = var("JOURNAL_DIR");
    if doc.is_ok() {
        return doc.ok().unwrap();
    }

    // Check for user document folder
    // ROOT PATH -> $HOME/Documents/Journal
    let user_dirs = UserDirs::new();
    let user_dirs = user_dirs.unwrap();
    let doc = user_dirs.document_dir();

    if doc.is_some() {
        let doc = doc.unwrap();
        return doc.to_str().unwrap().to_string();
    }

    // Check for XDG Config Dir
    // ROOT PATH -> $XDG_CONFIG_HOME/Journal
    let doc = var("XDG_CONFIG_HOME");
    if doc.is_ok() {
        return doc.ok().unwrap();
    }

    // Manual XDG Config Dir
    // ROOT PATH -> $HOME/.config/Journal
    let home = var("HOME");
    let doc = Path::new(&home.unwrap()).join(".config").to_str().unwrap().to_string();
    return doc;
}

/// Get Root Journal Directory
pub fn journal_dir() -> String {
    return format!("{}/Journal", get_document_path());
}

/// Get current day journal path
pub fn today_journal_path() -> String {
    let date = chrono::Utc::now();
    return format!(
        "{}/{}/{}/{}/JOURNAL-{}-{}-{}.md",
        journal_dir(),
        date.year(),
        date.month(),
        date.day(),
        date.year(),
        date.month(),
        date.day()
    );
}

/// Check if cli tool exists
pub fn binary_exists(bin: &str) -> bool {
    let exists =
        String::from_utf8(Command::new("which").arg(&bin).output().unwrap().stdout).unwrap();

    return exists != "";
}

/// Ensure journal directories exist
pub fn ensure_dirs() {
    let _ = fs::create_dir_all(format!("{}/Journal/Notes", get_document_path()));
}
