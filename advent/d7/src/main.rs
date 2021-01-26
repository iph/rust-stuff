extern crate nom;

use clap::{App, Arg};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1};
use nom::error::ErrorKind;
use nom::lib::std::collections::HashMap;
use nom::{multi::separated_list1, sequence::tuple, IResult};
use std::fs;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct BagRule<'a> {
    amount: i32,
    adjective: &'a str,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct BagDescription<'a> {
    bag_adjective: &'a str,
    fitted_rules: Vec<BagRule<'a>>,
}

// Bag traversal is gonna be extremely slow, but that's alright.
struct BagSolver<'a> {
    bags: Vec<BagDescription<'a>>,
    bags_arranged: HashMap<&'a str, BagDescription<'a>>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct InternalBagCounter<'a> {
    bag: BagDescription<'a>,
    count: i32,
}

struct Checkpoint<'a> {
    bags: Vec<BagDescription<'a>>,
    traversed: HashMap<&'a str, bool>,
}

impl<'a> BagSolver<'a> {
    fn new(content: &str) -> Result<BagSolver, &str> {
        let mut hash = HashMap::new();
        let mut vecs = Vec::new();
        for line in content.lines() {
            let bag = match fitted_bag_parser(line) {
                Ok(b) => b,
                Err(_e) => return Err(line),
            };
            vecs.push(bag.clone());
            hash.insert(bag.bag_adjective, bag);
        }

        return Ok(BagSolver {
            bags: vecs,
            bags_arranged: hash,
        });
    }

    fn solve(&self) -> u32 {
        let mut shines = 0;
        for bag in self.bags.clone() {
            let mut checkpoint = Checkpoint {
                bags: Vec::new(),
                traversed: HashMap::new(),
            };
            checkpoint.bags.push(bag.clone());
            checkpoint.traversed.insert(bag.bag_adjective, true);

            loop {
                if checkpoint.bags.len() == 0 {
                    break;
                }

                let loop_bag = checkpoint.bags.pop().unwrap();
                for rule in loop_bag.fitted_rules {
                    let fitted_bag = self.bags_arranged.get(rule.adjective).unwrap();

                    if !checkpoint.traversed.contains_key(fitted_bag.bag_adjective) {
                        checkpoint.bags.push(fitted_bag.clone());
                    }
                }

                checkpoint.traversed.insert(loop_bag.bag_adjective, true);
            }

            if checkpoint.traversed.contains_key("shiny gold") && bag.bag_adjective != "shiny gold"
            {
                println!("{}", bag.bag_adjective);
                shines += 1;
            }
        }

        return shines;
    }
    fn solve_reverse(&self) -> i32 {
        let mut all_bags: i32 = 0;
        let mut traversed = HashMap::new();
        let mut bags = Vec::new();

        let shiny = self.bags_arranged.get("shiny gold").unwrap();

        bags.push(InternalBagCounter {
            bag: shiny.clone(),
            count: 1,
        });
        loop {
            if bags.len() == 0 {
                break;
            }

            let current = bags.pop().unwrap();
            let current_adjective = current.bag.bag_adjective;
            all_bags += current.count;
            for rule in current.bag.fitted_rules {
                let rule_bag = self.bags_arranged.get(rule.adjective).unwrap();
                bags.push(InternalBagCounter {
                    bag: rule_bag.clone(),
                    count: current.count * rule.amount,
                })
            }
            traversed.insert(current_adjective, true);
        }
        return all_bags;
    }
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

    let (_, (origin, _, _, _, last)) = b(i)?;

    let mut all_rules = Vec::new();
    for rule in last.iter() {
        if let Some(x) = rule {
            all_rules.push(x.clone());
        }
    }
    return Ok(BagDescription {
        bag_adjective: origin,
        fitted_rules: all_rules,
    });
}

// Descriptors will be broken apart by an overarching separated list, and will be of the form:
//  <space><multiple_adjectives><"bag"|"bags">
// the bag|bags has a preceding space and so we will consume that token as well
fn parse_descriptors(i: &str) -> IResult<&str, Option<BagRule>> {
    if i == " no other bags." {
        return Ok(("", None));
    }

    let mut b = tuple((
        char(' '),
        alt((tag("no other"), digit1)),
        char(' '),
        take_until(" bag"),
        alt((tag(" bags"), tag(" bag"))),
    ));
    let (s, (_, quantity, _, adj, _)) = b(i)?;
    let quantity: i32 = match quantity {
        "no other" => 0,
        _ => match quantity.parse() {
            Ok(quantity) => quantity,
            Err(_) => {
                return Err(nom::Err::Failure(nom::error::make_error(
                    s,
                    ErrorKind::Digit,
                )))
            }
        },
    };

    Ok((
        s,
        Some(BagRule {
            amount: quantity,
            adjective: adj,
        }),
    ))
}

#[test]
fn bag_parser() {
    assert_eq!(
        BagDescription {
            bag_adjective: "light red",
            fitted_rules: vec![
                BagRule {
                    amount: 1,
                    adjective: "bright white"
                },
                BagRule {
                    amount: 2,
                    adjective: "muted yellow"
                }
            ]
        },
        fitted_bag_parser("light red bags contain 1 bright white bag, 2 muted yellow bags.")
            .unwrap()
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

    let solver = BagSolver::new(&contents).unwrap();
    let result = solver.solve_reverse();
    println!("{:?}", result-1);
}
