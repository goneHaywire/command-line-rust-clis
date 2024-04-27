use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    iter::Sum,
    ops::Add,
};

use anyhow::Result;

use crate::Cli;

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
        _ => Box::new(BufReader::new(File::open(file)?)),
    })
}

impl Stats {
    pub fn build(filename: &String) -> Result<Self> {
        let (l, w, b, c) = Stats::count(filename)?;
        Ok(Stats {
            filename: filename.into(),
            lines: l,
            words: w,
            bytes: b,
            chars: c,
        })
    }

    fn count(file: &str) -> Result<(usize, usize, usize, usize)> {
        let (mut l, mut w, mut b, mut c): (usize, usize, usize, usize) = (0, 0, 0, 0);
        let mut line_buf = String::new();
        let mut file = open(file)?;

        // my original solution
        // let mut was_last_byte_spacing = false;
        // let mut last_utf8_char: Vec<u8> = vec::Vec::with_capacity(4);

        loop {
            let line_bytes = file.read_line(&mut line_buf)?;
            if line_bytes == 0 {
                break;
            }

            l += 1;
            w += line_buf.split_whitespace().count();
            b += line_bytes;
            c += line_buf.chars().count();
            line_buf.clear();
        }

        // my original solution
        // for byte in open(file)?.bytes() {
        //     b += 1;
        //     let byte = byte?;
        //     last_utf8_char.push(byte);
        //
        //     if byte == b'\n' {
        //         l += 1;
        //     }
        //
        //     match (byte, was_last_byte_spacing) {
        //         (b' ' | b'\t' | b'\n', false) => {
        //             w += 1;
        //             was_last_byte_spacing = true;
        //         }
        //         (b' ' | b'\t' | b'\n', true) => was_last_byte_spacing = true,
        //         _ => {
        //             was_last_byte_spacing = false;
        //         }
        //     }
        //
        //     if String::from_utf8(last_utf8_char.clone()).is_ok() {
        //         c += 1;
        //         last_utf8_char.clear();
        //     }
        // }
        Ok((l, w, b, c))
    }

    pub fn print_stats(&self, args: &Cli) {
        if args.lines {
            print!("{:>8}", self.lines);
        }
        if args.words {
            print!("{:>8}", self.words);
        }
        if args.bytes {
            print!("{:>8}", self.bytes);
        }
        if args.chars {
            print!("{:>8}", self.chars);
        }
        if self.filename != "-" {
            println!(" {}", self.filename);
        } else {
            println!();
        }
    }
}

impl Add for Stats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stats {
            filename: "total".into(),
            lines: self.lines + rhs.lines,
            bytes: self.bytes + rhs.bytes,
            words: self.words + rhs.words,
            chars: self.chars + rhs.chars,
        }
    }
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            filename: "total".into(),
            lines: 0,
            words: 0,
            bytes: 0,
            chars: 0,
        }
    }
}

impl Sum for Stats {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Stats::default();

        for stat in iter {
            total = total + stat;
        }
        total
    }
}
