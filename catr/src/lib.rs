use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

#[derive(Parser, Debug)]
#[clap(
    author = "Nicholas Sack <nick@imns.dev>",
    version = "0.1.0",
    about = "Simple CLI cat program written in Rust.",
    long_about = None)]
pub struct Args {
    /// Input file(s)
    #[clap(default_value = "-", value_name="FILE")]
    files: Vec<String>,

    /// Number lines?
    #[clap(short, long, takes_value = false)]
    number_lines: bool,

    /// Number nonblank lines?
    #[clap(short = 'b', long, takes_value = false)]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Args::parse();
    Ok(Config {
        files: matches.files,
        number_lines: matches.number_lines,
        number_nonblank_lines: matches.number_nonblank_lines,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}