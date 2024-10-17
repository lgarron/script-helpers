use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::must_open_file::must_open_file;

pub fn read_lines(file_path: &Path) -> io::Lines<io::BufReader<File>> {
    io::BufReader::new(must_open_file(file_path)).lines()
}
