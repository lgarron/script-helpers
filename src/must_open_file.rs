use std::{fs::File, path::Path, process::exit};

pub fn must_open_file(file_path: &Path) -> File {
    let Ok(file) = File::open(file_path) else {
        eprintln!(
            "Could not find file to hash: {}",
            file_path.to_string_lossy()
        );
        exit(1);
    };
    file
}
