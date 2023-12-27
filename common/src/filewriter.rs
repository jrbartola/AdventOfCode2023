use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

pub fn write_file(path: &str, lines: Vec<String>) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    writer.flush()?;

    Ok(())
}
