use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Password {
    range: (i32, i32),
    letter: char,
    value: String,
}

impl Password {
    fn is_valid(&self) -> bool {
        let m: i32 = self.value.matches(self.letter).count() as i32;
        self.range.0 <= m && self.range.1 >= m
    }

    fn is_really_valid(&self) -> bool {
        1 == (self.letter == self.value.chars().nth((self.range.0 - 1)  as usize).unwrap()) as i32 
            + (self.letter == self.value.chars().nth((self.range.1 -1)  as usize).unwrap()) as i32
    }
}

impl FromStr for Password {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MATCHER: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
        };

        let parts = MATCHER.captures(s).unwrap();
        Ok(Password {
            range: (
                parts[1].parse::<i32>().unwrap(),
                parts[2].parse::<i32>().unwrap(),
            ),
            letter: parts[3].chars().next().unwrap(),
            value: parts[4].to_string(),
        })
    }
}

fn parse_input(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|line| Password::from_str(&line).unwrap())
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

    let vec = parse_input(&file_contents);
	let valid_passwords = vec.iter().filter(|p| p.is_valid()).count();
	let really_valid_passwords = vec.iter().filter(|p| p.is_really_valid()).count();

	println!("valid p1: {}", valid_passwords);
	println!("really_valid: {} ", really_valid_passwords);
}
