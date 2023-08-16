/*
A CLI tool that wraps PyO3 Embedded Python code.
*/
use clap::Parser;
use marco_python_cli::marco_python;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A CLI tool that wraps PyO3 Embedded Python code."
)]
struct Cli {
    #[clap(short, long)]
    input: String,
}

fn main() {
    let args = Cli::parse();
    let input = args.input;
    let output = marco_python(&input).unwrap();
    println!("{}", output);
}
