use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn load_input(file: &str) -> Vec<String> {
    let path = std::path::Path::new("inputs").join(file);
    println!("loading {}", path.display());
    let file = File::open(path).expect("Failed to open input file");

    BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Failed to read lines")
}