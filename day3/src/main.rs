use std::env;
use std::fs;

struct Map(Vec<Vec<char>>);

impl Map {
    fn go(&self, slope:(usize, usize)) -> usize {
        let mut trees = 0;
        for (idx, line) in self.0.iter().enumerate().step_by(slope.1) {
            if *line.iter().cycle().nth(idx * slope.0 / slope.1).unwrap() == '#' {
                trees += 1;
            }
        }

        trees
    }
}

fn parse_input(file_contents: &String) -> Vec<Vec<char>> {
    file_contents
        .lines()
        .map(|l| {
            l.chars().collect()
        })
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

    let map:Map = Map(parse_input(&file_contents));
    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    let tries: Vec<((usize, usize), usize)> = slopes.iter().map(|s| (*s, map.go(*s))).collect();

    println!("All tries : {:?}", tries);
    println!("First part : {}", tries[1].1);
    println!("Second part : {}", tries.iter().fold(1, |acc, value| acc * value.1));
}
