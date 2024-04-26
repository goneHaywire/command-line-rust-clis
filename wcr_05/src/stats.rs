use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use anyhow::Result;

#[derive(Debug)]
pub struct Stats {
    pub filename: String,
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    Ok(match file {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(&file)?)),
    })
}

impl Stats {
    pub fn build(filename: &String) -> Result<Self> {
        Ok(Stats {
            filename: filename.into(),
            lines: Stats::count_lines(filename)?,
            words: Stats::count_words(filename)?,
            bytes: Stats::count_bytes(filename)?,
            chars: 0,
        })
    }

    fn count_lines(filename: &String) -> Result<usize> {
        open(filename).map(|file| file.lines().count())
    }

    pub fn count_words(filename: &String) -> Result<usize> {
        let mut words = 0;
        let mut was_last_byte_spacing = false;
        for line in open(filename).map(|file| file.lines())? {
            let line = line?;
            if !&line.is_empty() {
                words += 1;
                for byte in line.bytes() {
                    println!("{words}");
                    match (byte, was_last_byte_spacing) {
                        (b' ' | b'\t', false) => {
                            words += 1;
                            was_last_byte_spacing = true;
                        }
                        (b' ' | b'\t', true) => was_last_byte_spacing = true,
                        _ => {
                            was_last_byte_spacing = false;
                            continue;
                        }
                    }
                }
            }
        }
        Ok(words)
    }

    pub fn count_bytes(filename: &String) -> Result<usize> {
        open(filename).map(|file| file.bytes().count())
    }
}
