use std::env;
use std::fs;
use std::process;
use regex::Regex;

struct Solver {}

impl Solver {
    fn solve(filename: String) -> i32 {
        let content = fs::read_to_string(filename).unwrap();
        return Solver::solve_contents(content)
    }

    fn solve_contents(contents: String) -> i32 {
        let matcher = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
        let lines = contents.lines();

        let mut valid_pws = 0;
        for line in lines {
            let caps = matcher.captures(line).unwrap();
            let lower:i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let upper: i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let letter = caps.get(3).unwrap().as_str();
            let content = caps.get(4).unwrap().as_str();

            let pw_policy = PasswordPolicy::new(letter.to_string(), lower, upper);
            if pw_policy.correct(content) {
                valid_pws+=1;
            }
        }

        return valid_pws;
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    letter: char,
    lower: i32,
    upper: i32,
}

impl PasswordPolicy {
    fn new(letter: String, lower: i32, upper: i32) -> PasswordPolicy {
        let letter: char = letter.chars().next().unwrap();
        return PasswordPolicy{letter, lower, upper};
    }

    fn correct(&self, content: &str) -> bool {
        let mut count = 0;
        let mut seen = false;
        for (position, c) in content.chars().enumerate().map(PasswordPolicy::one_indexed) {
            let p: i32 = position as i32;

            if c == self.letter && (p == self.lower || p == self.upper){
                count += 1;
                if seen {
                    return false
                }
                seen = true;    
            }
        }
        
        return count > 0;
    }


    fn one_indexed<T>((n, x): (usize, T)) -> (usize, T) {
        (n+1, x)
    }
}

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



fn main() {
    let config = Config::new(env::args())
    .unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let result = Solver::solve(config.filename);
    println!("Amount: {}", result)
}


#[test]
fn two_solutions() {
    let contents = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    let result = Solver::solve_contents(contents.to_string());
    assert_eq!(2, result)
}
