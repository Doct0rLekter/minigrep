use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments\n Usage: cargo run -- 'text' 'file.txt'");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let matched = search(&config.query, &contents);

    if matched == None {
        println!("    No matches found!")
    } else {
        let mut match_count = 0;
        for line in matched.unwrap() {
            println!("    Line match! \"{line}\"");
            match_count += 1;
        }
        println!("    {match_count} match(es) found")
    }


    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Option<Vec<&'a str>> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    if results.len() < 1 {
        return None
    }
    Some(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents).unwrap())
    }

    #[test]
    fn no_matches() {
        let query = "nuthin, honey";
        let contents = "\
Three
Blind
Mice";
        assert_eq!(None, search(query, contents))
    }
}
