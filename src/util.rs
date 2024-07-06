use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::{distributions::Alphanumeric, Rng};


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

pub fn get_words_by_dictionary(language: &str) -> Vec<String> {
    let words = match read_words_from_file(format!("dictionaries/{}.txt", language).as_str()) {
        Ok(words) => words,
        Err(_) => vec!["lorem".to_string(), "ipsum".to_string()],
    };

    words
}

pub fn generate_alphanumeric_tag(length: Option<usize>) -> String {
    let tag: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.unwrap_or(6))
        .map(char::from)
        .collect();

    tag
}
