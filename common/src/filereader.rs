use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub fn read_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        data.push(line);
    }

    Ok(data)
}
