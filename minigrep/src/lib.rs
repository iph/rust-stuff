use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;
    let result: Vec<&str>;

    if config.case_sensitive {
        result = search(&config.query, &contents);
    } else {
        result = search_insensitive(&config.query, &contents);
    }
    for line in result{
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found.push(line);
        }
    }

    found
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            found.push(line);
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct d
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Duct d
Pick three.";
        assert_eq!(vec!["safe, fast, productive.", "Duct d"], search_insensitive(query, contents))
    }
}