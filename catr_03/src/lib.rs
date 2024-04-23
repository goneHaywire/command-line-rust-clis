use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use anyhow::{Context, Ok, Result};
use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_args(files: Vec<String>) -> Result<()> {
    for file in files.into_iter().map(|file| match &file[..] {
        "-" => read_stdin().unwrap_or_else(|err| format!("{}", err)),
        _ => read_file(file).unwrap_or_else(|err| format!("{}", err)),
    }) {
        print!("{}", file);
    }
    Ok(())
}

fn read_file(file: String) -> Result<String> {
    let buf = std::fs::read_to_string(file).context("couldnt read file")?;
    Ok(buf)
}

fn read_stdin() -> Result<String> {
    let mut content = String::new();
    let stdin = std::io::stdin();

    stdin
        .read_line(&mut content)
        .context("couldn't read from stdin")?;
    Ok(content)
}

pub fn run(config: Config) -> Result<()> {
    read_args(config.files).unwrap();
    // dbg!(config);
    Ok(())
}

pub fn get_args() -> Result<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("me")
        .about("Rust cat")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Print line numbers")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Print line numbers for nonblank lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}
