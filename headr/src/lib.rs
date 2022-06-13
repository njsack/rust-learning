use clap::Parser;
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

#[derive(Parser, Debug)]
#[clap(
    author = "Nicholas Sack <nick@imns.dev>",
    version = "0.1.0",
    about = "A simple head program written in Rust",
    long_about = None
)]
pub struct Args {
    // Number of lines
    #[clap(short = "n", long, default_value = "10")]
    lines: usize,

    // Number of bytes
    #[clap(
        short = "c", long,
        takes_value = true,
        value_name = "BYTES"
        conflicts_with = "lines"
    )]
    bytes: Option<usize>,

    // Input file(s)
    #[clap(value_name = "FILE", multiple = true, default_value = "-")]
    files: Vec<String>,
}