use std::collections::HashSet;
use std::env;
use std::fs;

fn parse_input(input: &str) -> Vec<i64> {
    println!("{:?}", input);
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn twosum(report: &Vec<i64>) -> Option<(i64, i64)> {
    let mut m: HashSet<i64> = HashSet::with_capacity(report.len());
    for expense in report.iter() {
        match m.contains(expense) {
            true => {
                return Some((2020 - *expense, *expense));
            }
            false => {
                m.insert(2020 - expense);
            }
        }
    }

    return None;
}

fn threesum(report: &Vec<i64>) -> Option<(i64, i64, i64)> {
    for i in 0..report.len() - 2 {
        for j in i + 1..report.len() - 1 {
            for k in j + 1..report.len() {
                if report[i] + report[j] + report[k] == 2020 {
                    return Some((report[i], report[j], report[k]));
                }
            }
        }
    }
    None
}

fn part1(report: &Vec<i64>) -> i64 {
    match twosum(report) {
        Some((a, b)) => {
            println!("Found the matching value {} + {} = 2020 ", a, b);
            a * b
        }
        None => 0,
    }
}

fn part2(report: &Vec<i64>) -> i64 {
    match threesum(report) {
        Some((a, b, c)) => {
            println!("Found the matching value {} + {} + {}= 2020 ", a, b, c);
            a * b * c
        }
        None => 0,
    }
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

	println!("{:?}", part1(&vec));
	println!("{:?}", part2(&vec));
}
