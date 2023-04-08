// Set clippy to pedantic
#![warn(clippy::pedantic)]

use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let contents = read_file(&config.file_path);

    println!("Searching for: {}\n", config.query);
    println!("In file: {}\n", config.file_path);
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Could not read the file")
}
