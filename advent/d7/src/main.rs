extern crate nom;

use clap::{App, Arg};
use std::fs;
use nom::{IResult, sequence::tuple, multi::many1};
use nom::character::{is_digit};
use nom::bytes::complete::tag;
use nom::bytes::complete::is_space;
use nom::bytes::complete::take_until;

fn steal_space(input: &str) -> IResult<&str, &str> {
    return is_space(input);
}

fn takeBag(i: &str) -> IResult<&str, &str> {
    let mut b = tuple((take_until("bag"), take_until("contain"), many1()));
    let mut  l = tuple((tag("hi"), tag("bye")));
}

#[test]
fn name() {
    println!("{:?}",takeBag("light red bags contain 1 bright white bag, 2 muted yellow bags."));
}

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
    
}
