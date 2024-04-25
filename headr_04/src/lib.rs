use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use anyhow::{Ok, Result};
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
    // println!("{:?} {}", args.bytes, args.lines);

    for file in args.files.iter() {
        if files_len > 1 {
            println!("==> {file} <==");
        }
        let handle = open(file)?;
        if let Some(bytes) = args.bytes {
            let mut output = String::new();
            BufReader::new(handle.take(bytes)).read_to_string(&mut output)?;
            print!("{}", output);
        } else {
            let lines = handle.lines().take(args.lines.try_into()?);
            // while let Some(line) = handle.lines().take(args.lines.try_into()?) {
            //     println!("{}", line)
            // }
            // while let Some(line) = lines.next() {
            //     println!("{:?}", line?);
            // } else {
            //     println!("");
            // }
            // for line in lines.next() {
            // }

            for line in lines {
                println!("{}", line?);
            }
            // if lines.clone().count() == 0 {
            //     println!("");
            // } else {
            //     for line in lines {
            //         println!("{}", line?);
            //     }
            // }
        }
    }
    Ok(())
}

pub fn get_args() -> Result<Args> {
    Ok(Args::try_parse()?)
}
