use clap::{App, Arg};
use std::fs;

#[derive(Debug, PartialEq, PartialOrd)]
struct Passenger {
    id: i32,
    row: i32,
    column: i32,
}

impl Passenger {
    fn parse(contents: &str) -> Result<Passenger, &'static str> {
        let mut lower = 0;
        let mut upper = 127;

        let mapping: Vec<char> = contents.chars().collect();
        for i in (0..7) {
            let current = (upper + lower) / 2;
            let remainder = (upper + lower) % 2;
            match mapping.get(i).unwrap() {
                'F' => upper = current,
                'B' => lower = current + remainder,
                _ => return Err("Didn't have the correct codes"),
            };
        }

        let partition = lower;
        let mut lower = 0;
        let mut upper = 7;
        for i in (7..10) {
            let current = (upper + lower) / 2;
            let remainder = (upper + lower) % 2;
            match mapping.get(i).unwrap() {
                'L' => upper = current,
                'R' => lower = current + remainder,
                _ => return Err("Didn't have the correct codes"),
            };
        }

        let seat_col = lower;
        return Ok(Passenger {
            id: 8 * partition + seat_col,
            row: partition,
            column: seat_col,
        });
    }
}

#[test]
fn test_samples() {
    assert_eq!(
        Passenger {
            id: 357,
            row: 44,
            column: 5
        },
        Passenger::parse("FBFBBFFRLR").unwrap()
    );
    assert_eq!(
        Passenger {
            id: 567,
            row: 70,
            column: 7
        },
        Passenger::parse("BFFFBBFRRR").unwrap()
    );
    assert_eq!(
        Passenger {
            id: 119,
            row: 14,
            column: 7
        },
        Passenger::parse("FFFBBBFRRR").unwrap()
    );
    assert_eq!(
        Passenger {
            id: 820,
            row: 102,
            column: 4
        },
        Passenger::parse("BBFFBBFRLL").unwrap()
    );
}

#[derive(Debug, PartialEq)]
struct Airplane {
    passengers: Vec<Passenger>,
}

impl Airplane {
    fn parse(contents: &str) -> Result<Airplane, &'static str> {
        let mut passengers = Vec::new();
        for line in contents.lines() {
            let passenger = Passenger::parse(line)?;
            passengers.push(passenger);
        }
        return Ok(Airplane {
            passengers: passengers,
        });
    }

    fn max_id(&self) -> Option<&Passenger> {
        let mut iter = self.passengers.iter();
        let mut max: Option<&Passenger> = iter.next();
        for passenger in iter {
            if let Some(current_max) = max {
                if passenger.id > current_max.id {
                    max = Some(passenger);
                }
            }
        }

        return max;
    }
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
    let mut airplane = Airplane::parse(&contents).unwrap();
    println!("{}", airplane.max_id().unwrap().id);
    airplane.passengers.sort_by((|a, b| a.id.cmp(&b.id)));
    let mut iter = airplane.passengers.iter();
    let mut previous_passenger = iter.next().unwrap();
    for passenger in iter {
        let next_passenger_id = previous_passenger.id + 1;

        if passenger.id != next_passenger_id {
            println!("Missing seat {:?}", next_passenger_id);
        }

        previous_passenger = passenger;
    }
}
