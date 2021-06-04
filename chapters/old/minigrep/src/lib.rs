use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Skipped because it's always the name of the program.
        args.next();

        let query = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get a filename string"),
        };

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
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter( | line | line.to_lowercase().contains(&query))
        .collect()
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