use std::{collections::HashMap};
use std::fs;
use std::env;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct Passport(HashMap<String, String>);

type Pred = fn(Option<&String>) -> bool;

impl Passport {
    fn is_valid(&self) -> bool {
        lazy_static! {
            static ref HEIGHT_REGEX: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
            static ref HAIR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}").unwrap();
            static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
            static ref VALIDATORS: &'static [(&'static str, Pred)] = &[
                ("byr", |value| { 
                    let byr = value.unwrap_or(&"".to_string()).parse::<usize>().unwrap_or(0);
                    byr >= 1920 && byr <=2002
                }),
                ("iyr", |value| { 
                    let byr = value.unwrap_or(&"".to_string()).parse::<usize>().unwrap_or(0);
                    byr >= 2010 && byr <=2020
                }),
                ("eyr", |value| { 
                    let byr = value.unwrap_or(&"".to_string()).parse::<usize>().unwrap_or(0);
                    byr >= 2020 && byr <=2030
                }),
                ("hgt", |value| {
                    match HEIGHT_REGEX.captures(&value.unwrap_or(&"".to_string())) {
                        Some(caps) => {
                            let h = caps[1].parse::<usize>().unwrap();
                            match &caps[2] {
                                "cm" => h >= 150 && h <= 193,
                                "in" => h >= 59 && h <= 76,
                                _ => false
                            }
                        }
                        None => false
                    }
                }),
                ("hcl", |value| HAIR_REGEX.is_match(&value.unwrap_or(&"".to_string())) ),
                ("ecl", |value| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.unwrap_or(&"".to_string()).as_str()) ),
                ("pid", |value| PID_REGEX.is_match(&value.unwrap_or(&"".to_string()))),
                ("cid", |_| true )
                ];
            static ref IGNORED: String = String::from("cid");
        };

        VALIDATORS.iter()
            .all(|(key, pred)| pred(self.0.get(&key.to_string())))
    }
}

fn parse(input: &String) -> Vec<Passport> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .as_slice()
        .split(|l| l.is_empty())
        .map(|x| 
            x
                .iter()
                .map(|e| e.split_whitespace())
                .flatten()
                .fold(Passport(HashMap::new()), |mut p, entry| {
                    match entry.split(":").collect::<Vec<&str>>().as_slice() {
                        [key, value] => { p.0.insert(key.to_string(), value.to_string()); },
                        _ => println!("Invalid entry {}", entry)
                    }
                    p
                })
        )
        .collect()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let valid_passports:Vec<Passport> = parse(&file_contents)
            .iter()
            .filter(|p| p.is_valid())
            .map(|p| p.clone())
            .collect();

    println!("Valid passports : {}", valid_passports.len());
}
