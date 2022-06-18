use crate::etc;
use std::env::var;
use std::process::Command;

pub fn write_journal(note: Option<&str>) {
    // Open Journal or Note
    let mut note_path = etc::today_journal_path(); // Open new journal
    if note.is_some() {
        // Open Note
        note_path = format!("{}/Notes/{}.md", etc::journal_dir(), note.unwrap());
    }
    etc::ensure_parents_exist(note_path.to_string());

    // Open Editor
    Command::new(var("EDITOR").ok().unwrap_or_else(|| "nano".to_string()))
        .arg(&note_path)
        .status()
        .unwrap();
}
