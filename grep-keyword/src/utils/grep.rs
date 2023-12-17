use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn find_keyword(log_file_path: &str, keyword: &str) -> io::Result<Vec<String>> {
    let file = File::open(log_file_path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    let mut found = false;

    for line in reader.lines() {
        let line = line?;

        if line.contains(keyword) {
            found = true;
        }

        lines.push(line);

        if lines.len() > 40 {
            lines.remove(0);
        }

        if found && lines.len() == 40 {
            return Ok(lines);
        }
    }
    Ok(lines)
}