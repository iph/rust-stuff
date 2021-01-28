use std::env;
use std::fs;
use std::process;

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
    'outer: for i in lines.iter() {
        for j in lines.iter() {
            for k in lines.iter(){
                if i+j+k == 2020 {
                    println!("{}, {}, {}: {}", i, j, k, (i*j*k));
                    break 'outer;
                }
            }
        }
    }
}
