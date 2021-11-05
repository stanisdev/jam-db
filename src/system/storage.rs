use std::fs::OpenOptions;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};
use std::io::{self, SeekFrom};
use std::io::prelude::*;

pub struct Storage {}

impl Storage {
    pub fn new() -> Storage {
        Storage {}
    }

    pub fn read(&self) -> io::Result<()> {
        let path = Path::new("./src/storage/areas"); // @todo: change this
        let file = OpenOptions::new().read(true).write(true).open(&path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Err(message) = line {
                panic!("{}", message);
            }
            let value = line.unwrap();
            println!("{}", value);
        }
        Ok(())
    }

    pub fn write(&self) -> io::Result<()> {
        let path = Path::new("./src/storage/areas"); // @todo: change this
        let mut file = OpenOptions::new().read(true).write(true).open(&path)?;

        file.seek(SeekFrom::End(0))?;
        let mut writer = BufWriter::new(file);
        writer.write("+".as_bytes())?;
        
        writer.flush()?;
        Ok(())
    }
}
