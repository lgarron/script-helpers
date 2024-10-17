use std::path::Path;

use crate::read_lines::read_lines;

pub fn any_line_starts_with(source: &Path, prefix: &str) -> bool {
    for line in read_lines(source).map_while(Result::ok) {
        if line.starts_with(prefix) {
            return true;
        };
    }
    false
}
