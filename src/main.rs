use minigrep as lib;
use std::{env, process};

fn main() {
    let argv: Vec<String> = env::args().collect();

    let config = lib::Config::build(&argv).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        eprintln!("Failed to read file contents: {}", e);
        process::exit(1)
    }
}
