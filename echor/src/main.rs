use clap::Parser;

/// Simple echo program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Omit newline on print
    #[clap(short = 'n', long)]
    no_newline: bool,

    /// Input to echo
    #[clap(required = true, min_values = 1)]
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let text = args.text.join(" ");
    let omit_newline = args.no_newline;

    print!("{}{}", text, if omit_newline { "" } else { "\n" });
}
