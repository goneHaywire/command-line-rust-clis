use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of 'head'
pub struct Args {
    /// Input file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'),
        long,
        default_value = "10",
        value_name = "LINES",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// number of bytes
    #[arg(
        short('c'),
        long,
        value_name = "BYTES",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    Ok(match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    })
}

pub fn run(args: Args) -> Result<()> {
    let files_len = args.files.len();

    for (i, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(e) => {
                eprintln!("{filename}: {e}");
            }
            Ok(mut handle) => {
                if i != 0 {
                    println!();
                };
                if files_len > 1 {
                    println!("==> {filename} <==");
                }
                if let Some(bytes) = args.bytes {
                    let mut buf = vec![0; bytes as usize];
                    let bytes_read = handle.take(bytes).read(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let len = handle.read_line(&mut line)?;
                        print!("{line}");
                        if len == 0 {
                            break;
                        }
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> Result<Args> {
    Ok(Args::try_parse()?)
}
