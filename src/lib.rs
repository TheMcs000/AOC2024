use std::fs::File;
use std::io::{self, Read};

pub fn read_input(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?; // Replace with your file path
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
