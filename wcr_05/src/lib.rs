mod stats;

use std::io::Bytes;

use anyhow::Result;
use clap::Parser;

use crate::stats::Stats;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of 'wc'
pub struct Cli {
    #[arg(default_value = "-", value_name = "FILES")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

pub fn run() -> Result<()> {
    let args = get_args();
    // dbg!(&args);
    for filename in args.files.iter() {
        match Stats::build(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(stats) => {
                if args.lines {
                    print!("{:>8}", stats.lines);
                }
                if args.words {
                    print!("{:>8}", stats.words);
                }
                if args.bytes {
                    print!("{:>8}", stats.bytes);
                }
                if args.chars {
                    print!("{:>8}", stats.chars);
                }
                println!(" {}", filename);
                dbg!(stats);
            }
        }
    }
    Ok(())
}

pub fn get_args() -> Cli {
    let args = Cli::parse();
    let (lines, words, bytes, chars) = match (args.lines, args.words, args.bytes, args.chars) {
        (false, false, false, false) => (true, true, true, false),
        (l, w, c, m) => (l, w, c, m),
    };
    Cli {
        files: args.files,
        lines,
        words,
        bytes,
        chars,
    }
}
