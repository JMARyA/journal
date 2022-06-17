use chrono::Datelike;
use directories::UserDirs;
use std::fs;
use std::process::Command;

/// Get Root Journal Directory
pub fn journal_dir() -> String {
    let user_dirs = UserDirs::new();
    let user_dirs = user_dirs.unwrap();
    let docs = user_dirs.document_dir().unwrap();
    return format!("{}/Journal", docs.to_str().unwrap());
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
    let user_dirs = UserDirs::new();
    let user_dirs = user_dirs.unwrap();
    let docs = user_dirs.document_dir().unwrap();
    let _ = fs::create_dir_all(format!("{}/Journal/Notes", docs.to_str().unwrap()));
}
