use clap::{App, Arg};
use std::fs;
use std::collections::HashMap;

fn main() {
    let matches = App::new("Reader for advent")
        .version("1.0")
        .author("Sean Tyler Myers <seanmyers0608@gmail.com>")
        .about("Reads advent calendar programs")
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let txt_location: &str = matches.value_of("INPUT").unwrap();
    let contents = fs::read_to_string(txt_location).unwrap();
    
    let mut counts: HashMap<char, bool> = HashMap::new();
    let mut full_count = 0;
    let lines:Vec<&str> = contents.lines().collect();
    for line in lines.iter() {
        if line.to_owned() == "" {
            full_count += counts.len();
            counts = HashMap::new();
        }

        for c in line.chars(){
            counts.insert(c, true);
        }
    }

    full_count += counts.len();
    println!("counts {}", full_count);

    // harder problem
    let contents = fs::read_to_string(txt_location).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let mut counts: HashMap<char, u32> = HashMap::new();
    let mut people = 0;
    full_count = 0;
    for line in lines.iter() {
        if line.to_owned() == "" {
            for (_,amt) in counts {
                if amt == people {
                    full_count += 1;
                }
            }
            counts = HashMap::new();
            people = 0;
            continue;
        }

        for c in line.chars(){
            let char_count = counts.get(&c).unwrap_or(&0);
            let total = char_count + 1;
            counts.insert(c, total);
        }
        people += 1;
        println!("people {}, {}", people, line)
    }
    for (_,amt) in counts {
            if amt == people {
                full_count += 1;
            }
    }

    println!("Other counts {}", full_count)
}