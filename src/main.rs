mod cli;
use std::process::exit;

use clap::Parser;
use cli::Cli;
use colored::Colorize;
use knightmare::error::Error;

fn main() {
    match run() {
        Ok(v) => exit(v),
        Err(e) => {
            println!("{}", e.to_string().red())
        }
    }
}

fn run() -> Result<i32, Error> {
    let cli = Cli::parse();

    cli.run()
}
