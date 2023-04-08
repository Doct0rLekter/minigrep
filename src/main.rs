// Set clippy to pedantic
#![warn(clippy::pedantic)]

use std::env; // For env::args()
use std::process; // For process::exit()

use minigrep::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}\n", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Apllication error: {e}");
        process::exit(1);
    }
}

