use sha2::{Digest, Sha256};
use std::io::{self};
use std::path::Path;

use crate::must_open_file::must_open_file;

pub fn sha256_hash_file_to_string(file_path: &Path) -> String {
    let mut hasher = Sha256::new();
    if let Err(err) = io::copy(&mut must_open_file(file_path), &mut hasher) {
        eprintln!("Could not hash file: {}", file_path.to_string_lossy());
        eprintln!("Error: {}", err);
    };
    let hash = hasher.finalize();
    hex::encode(hash)
}
