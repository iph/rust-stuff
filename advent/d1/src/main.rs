use std::env;
use std::fs;
use std::process;
use std::collections::HashMap;


struct Config {
    filename: String,
}

impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        args.next();

        let filename = match args.next() {
            Some(val) => val,
            None => return Err("Didn't get a filename string"),
        };

        Ok(Config{filename})
    }

}

fn get_lines(config: Config) -> Result<Vec<i32>, &'static str> {
    let contents = fs::read_to_string(&config.filename).unwrap();
    return Ok(contents.lines()
            .map(|j|{j.parse::<i32>().unwrap()})
            .collect())
}

fn main() {
    let config = Config::new(env::args())
        .unwrap_or_else(|err|{
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    
    let lines = get_lines(config).unwrap();
    let mut check: HashMap<i32, bool>  = HashMap::new();
    for i in lines { 
        let other = 2020 - i;
        
        match check.get(&other) {
            Some(_) => println!("Found a match {}!", other*i),
            _ => {},
        }
        check.insert(i, true);
    }
}
