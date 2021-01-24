extern crate nom;

use clap::{App, Arg};
use nom::branch::alt;
use nom::character::{is_digit};
use nom::error::{FromExternalError};
use nom::bytes::complete::{tag, take_until, is_a, take_while1};
use nom::character::complete::{char, digit1};
use nom::{multi::separated_list1, sequence::tuple, IResult};
use std::fs;

#[derive(Debug, PartialEq, PartialOrd)]
struct BagRule<'a>{
    amount: i32,
    adjective: &'a str
}

#[derive(Debug, PartialEq, PartialOrd)]
struct BagDescription<'a> {
    bag_adjective: &'a str,
    fitted_rules: Vec<BagRule<'a>>,
}
// fitted_bag_parser will parse a string of the form:
// <adjectives> (bag|bags) contain<LOOP>
// where loop is of the form " <adjectives> (bag|bags)" with a comma between them.
fn fitted_bag_parser(i: &str) -> Result<BagDescription, nom::Err<nom::error::Error<&str>>> {
    let mut b = tuple((
        // In the word "light red bags contains", we want to capture "light red", and consume up to and including contains
        take_until(" bag"),
        alt((tag(" bags"), tag(" bag"))),
        take_until("contain"),
        tag("contain"),
        // At this point, we start the loop of "<space><adjectives> (bag|bags)", separated by commas
        separated_list1(tag(","), parse_descriptors),
    ));

    if let Err(h) = b(i) {
        return Err(h);
    }

    let (s, (origin, _, _, _, last)) = b(i)?;
    println!("{}", origin);
    println!("{:?}", last);
    println!("{}", s);
    return Ok(BagDescription {
        bag_adjective: origin,
        fitted_rules: last,
    });
}

// Descriptors will be broken apart by an overarching separated list, and will be of the form:
//  <space><multiple_adjectives><"bag"|"bags">
// the bag|bags has a preceding space and so we will consume that token as well
fn parse_descriptors(i: &str) -> IResult<&str, BagRule> {
    let mut b = tuple((
        char(' '),
        digit1,
        char(' '),
        take_until(" bag"),
        alt((tag(" bags"), tag(" bag"))),
    ));
    let (s, (_, quantity, _, adj, _)) = b(i)?;

    let quantity:i32 = quantity.parse::<i32>().unwrap();
    Ok((s, BagRule{amount: quantity, adjective: adj}))
}

#[test]
fn name() {
    println!(
        "{:?}",
        fitted_bag_parser("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap()
    );
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

    for line in contents.lines(){ 
        println!("{:?}",fitted_bag_parser(line));
    }
}
