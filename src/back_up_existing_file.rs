use std::{fs::rename, path::Path};

use chrono::Local;

pub fn back_up_existing_file(file: &Path, extension: &str) {
    if file.exists() {
        let date = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let backup_file_name = format!("{}.bak.{}.{}", file.to_string_lossy(), date, extension);
        println!("➡️ File already exists. Moving to: {}", backup_file_name);
        rename(file, backup_file_name).expect("Could not back up existing target file.");
    }
}
