use std::io::{self, BufReader};
use std::fs::File;

pub fn file_reader(file_path: &str) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}