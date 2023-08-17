/*
CLI interface for Polars.  Calls lib.rs
*/
use clap::Parser;
use polars_cli::calculate;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A CLI tool that wraps Polars code."
)]
struct Cli {
}

fn main() {
    let _args = Cli::parse();
    let result = calculate().unwrap();
    println!("{:?}", result);
}

