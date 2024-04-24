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

pub fn run(args: Args) -> Result<()> {
    let files_len = args.files.len();
    let lines = args.lines;
    let bytes = args.bytes.unwrap_or(0);
    println!("{files_len} {lines} {bytes}");
    Ok(())
}

pub fn get_args() -> Result<Args> {
    Ok(Args::try_parse()?)
}
