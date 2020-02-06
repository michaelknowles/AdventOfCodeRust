use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_file(file_name: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(file_name).expect("Can't read file");
    let reader = BufReader::new(file);
    reader.lines()
          .map(|line| line.expect("Couldn't read line"))
          .collect()
}