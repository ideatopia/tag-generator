use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub fn read_words_from_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let buf = io::BufReader::new(file);
    let mut words = Vec::new();

    for line in buf.lines() {
        let line = line?;
        words.push(line.trim().to_string());
    }

    Ok(words)
}
