use std::fs::{ self, File };
use std::io::{ self, BufRead, Lines, Result, BufReader };
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_line<P>(filename: P) -> Result<String> where P: AsRef<Path> {
    let line = fs::read_to_string(filename)?;
    Ok(line)
}