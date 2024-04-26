mod stats;

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

    let total: Stats = args
        .files
        .iter()
        .map(|filename| (filename, Stats::build(filename)))
        .inspect(|(filename, stats)| match stats {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(stats) => stats.print_stats(&args),
        })
        .filter(|(_, stats)| Result::is_ok(stats))
        .map(|(_, stats)| stats.unwrap())
        .sum();

    if args.files.len() > 1 {
        total.print_stats(&args);
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
