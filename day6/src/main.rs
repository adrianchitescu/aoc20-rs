use std::env;
use std::fs;
use itertools::Itertools;

fn parse(input: &String) -> Vec<Vec<String>>{
    input
        .lines()
        .collect::<Vec<&str>>()
        .as_slice()
		.split(|l| l.is_empty())
		.map(|l| l.iter().map(|a| a.trim().to_string()).collect())
		.collect()
}

fn yes_answers(answers: &Vec<Vec<String>>) -> usize {
	answers.iter().fold(0, |s, anss| {
		s + anss.iter().map(|a| a.chars()).flatten().filter(|c| !c.is_whitespace()).unique().count()
	})
}

fn everyone_yes_answers(answers : &Vec<Vec<String>>) -> usize {
	answers.iter().fold(0, |s, anss| {
		s +  ('a'..='z').filter(|q| anss.iter().all(|a| a.contains(&q.to_string()))).count()
	})
}

fn main() {
	let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
	});
	
	let answers = parse(&file_contents);

	println!("Yes : {}", yes_answers(&answers));
	println!("All yes : {}", everyone_yes_answers(&answers));
}

#[cfg(test)]
mod tests {
	use crate::{parse, yes_answers, everyone_yes_answers};

	#[test]
	fn parse_test() {
		assert_eq!(parse(&"abc".to_string()), vec![vec!["abc"]]);
		assert_eq!(parse(&
			r"abc

			a
			b
			c".to_string()),
			 vec![vec!["abc"], vec!["a","b","c"]]);
		assert_eq!(parse(&
			r"abc

			a
			b
			c

			ab
			ac

			a
			a
			a
			a

			b".to_string()), 
			vec![vec!["abc"], vec!["a","b","c"], vec!["ab", "ac"], vec!["a", "a", "a", "a"], vec!["b"]]);
	}

	#[test]
	fn sums() {
		let answers = parse(&
			r"abc

			a
			b
			c

			ab
			ac

			a
			a
			a
			a

			b".to_string());
		assert_eq!(yes_answers(&answers), 11);
		assert_eq!(everyone_yes_answers(&answers), 6)
	}
}