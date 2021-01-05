use clap::{App, Arg};
use std::fs;

#[derive(Debug, PartialEq)]
struct Passport {
    birth: String,
    issue: String,
    expiration: String,
    height: String,
    hair: String,
    eye: String,
    id: String,
    cid: Option<String>,
}

#[derive(Debug, PartialEq)]
enum Reason {
    BIRTH_INVALID,
    ISSUE_INVALID,
    EXPIRATION_INVALID,
    HEIGHT_INVALID,
    HAIR_INVALID,
    EYE_INVALID,
    ID_INVALID,
    MISSING_FIELDS,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            birth: String::new(),
            issue: String::new(),
            expiration: String::new(),
            height: String::new(),
            hair: String::new(),
            eye: String::new(),
            id: String::new(),
            cid: None,
        }
    }

    fn validate(&self) -> Result<(), Reason> {
        let empty = String::new();
        let is_empty = self.birth != empty
            && self.issue != empty
            && self.expiration != empty
            && self.height != empty
            && self.hair != empty
            && self.eye != empty
            && self.id != empty;

        if !is_empty {
            return Err(Reason::MISSING_FIELDS);
        }

        let byear = match self.birth.parse::<i32>() {
            Ok(year) => year,
            Err(_) => return Err(Reason::BIRTH_INVALID),
        };

        let birthday_year_valid = byear >= 1920 && byear <= 2002;

        if !birthday_year_valid {
            return Err(Reason::BIRTH_INVALID);
        }

        let issue_year = match self.issue.parse::<i32>() {
            Ok(year) => year,
            Err(_) => return Err(Reason::ISSUE_INVALID),
        };

        let issue_year_valid = issue_year >= 2010 && issue_year <= 2020;

        if !issue_year_valid {
            return Err(Reason::ISSUE_INVALID);
        }

        let expiration_year = match self.expiration.parse::<i32>() {
            Ok(year) => year,
            Err(_) => return Err(Reason::EXPIRATION_INVALID),
        };

        let expiration_year_valid = expiration_year >= 2020 && expiration_year <= 2030;

        if !expiration_year_valid {
            return Err(Reason::EXPIRATION_INVALID);
        }

        let eye_color_valid = match &self.eye[..] {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        };

        if !eye_color_valid {
            return Err(Reason::EYE_INVALID);
        }

        let id_valid = self.id.len() == 9 && self.id.chars().all(char::is_numeric);
        if !id_valid {
            return Err(Reason::ID_INVALID);
        }

        if !self.hair_valid() {
            return Err(Reason::HAIR_INVALID);
        }

        if !self.height_valid() {
            return Err(Reason::HEIGHT_INVALID);
        }

        return Ok(());
    }

    fn height_valid(&self) -> bool {
        let mut chars: Vec<char> = self.height.chars().collect();

        if chars.len() <= 3 {
            return false;
        }

        let last_char = chars.pop();
        let second_last_char = chars.pop();

        let t: String = vec![second_last_char.unwrap(), last_char.unwrap()]
            .into_iter()
            .collect();
        match &t[..] {
            "cm" => {}
            "in" => {}
            _ => return false,
        };

        let size: String = chars.into_iter().collect();
        let size = match size.parse::<i32>() {
            Ok(val) => val,
            Err(_) => return false,
        };

        if t == "cm" {
            return size >= 150 && size <= 193;
        } else {
            return size >= 59 && size <= 76;
        }
    }

    fn hair_valid(&self) -> bool {
        if self.hair.len() != 7 {
            return false;
        }

        let mut remainder = self.hair.chars();

        let first = remainder.next();
        let hash = match first {
            Some(val) => val,
            None => return false,
        };

        if hash != '#' {
            return false;
        }

        remainder.all(|c| {
            char::is_numeric(c)
                || match c {
                    'a'..='f' => true,
                    'A'..='F' => true,
                    _ => false,
                }
        })
    }
}

#[test]
fn test_hair() {
    let mut pass = Passport::new();
    pass.hair = "#ABCD12".to_string();
    assert_eq!(true, pass.hair_valid());

    pass.hair = "#ABCDEF".to_string();
    assert_eq!(true, pass.hair_valid());

    pass.hair = "ABCDEF".to_string();
    assert_eq!(false, pass.hair_valid());

    pass.hair = "-ABCDEF".to_string();
    assert_eq!(false, pass.hair_valid());

    pass.hair = "#123abc".to_string();
    assert_eq!(true, pass.hair_valid());
}

#[test]
fn test_height() {
    let mut pass = Passport::new();
    pass.height = "150cm".to_string();
    assert_eq!(true, pass.height_valid());

    pass.height = "193cm".to_string();
    assert_eq!(true, pass.height_valid());

    pass.height = "59in".to_string();
    assert_eq!(true, pass.height_valid());

    pass.height = "76in".to_string();
    assert_eq!(true, pass.height_valid());

    pass.height = "77in".to_string();
    assert_eq!(false, pass.height_valid());

    pass.height = "58in".to_string();
    assert_eq!(false, pass.height_valid());

    pass.height = "59inc".to_string();
    assert_eq!(false, pass.height_valid());
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
    println!("{}", solve(&contents).unwrap());
}

fn solve(contents: &str) -> Result<i32, String> {
    let mut strs: Vec<String> = Vec::new();
    let mut built_str = String::new();
    let mut last_char = ' ';
    for c in contents.chars() {
        if c == '\n' && last_char == '\n' {
            strs.push(built_str.trim().to_string());
            built_str = String::new();
        }

        if c == '\n' {
            built_str.push(' ');
        } else {
            built_str.push(c);
        }

        last_char = c;
    }
    strs.push(built_str);
    let mut valid_passports = 0;
    for line in strs {
        let all_passport_data = line.trim().split(" ");
        let mut passport = Passport::new();
        for passport_string in all_passport_data {
            let key_value: Vec<&str> = passport_string.split(":").collect();

            if key_value.len() != 2 {
                return Err(format!(
                    "Not properly formatted key-value pairs {}",
                    passport_string
                ));
            }
            let key: &str = key_value.get(0).unwrap();
            let value = key_value.get(1).unwrap();

            match key {
                "byr" => passport.birth = value.to_string(),
                "iyr" => passport.issue = value.to_string(),
                "eyr" => passport.expiration = value.to_string(),
                "hgt" => passport.height = value.to_string(),
                "hcl" => passport.hair = value.to_string(),
                "pid" => passport.id = value.to_string(),
                "ecl" => passport.eye = value.to_string(),
                "cid" => passport.cid = Some(value.to_string()),
                _ => return Err(format!("Not an acceptable passport {}", key)),
            }
        }

        match passport.validate() {
            Ok(_) => valid_passports += 1,
            Err(_) => {},
        }
    }
    return Ok(valid_passports);
}

#[test]
fn test_content_parsing() {
    let contents = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    let ret = solve(&contents);
    println!("{}", ret.unwrap())
}
