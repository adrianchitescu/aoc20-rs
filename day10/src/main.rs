extern crate utils;
use utils::utils::*;

fn find_all_adapter_ways(adapters:&Vec<usize>, ) -> usize {
	let mut f: Vec<usize> = vec![0; adapters.len()];

	f[0] = 1;
	for i in 1..adapters.len() {
		let mut lower_bound = 0;
		if i > 3 {
			lower_bound = i - 3;
		}
		for j in lower_bound..i {
			if adapters[i] - adapters[j as usize] <= 3 {
				f[i] += f[j as usize];
			}
		}
	}

	f[f.len() - 1]
}

fn find_distribution(adapters:&Vec<usize>, ) -> (usize, usize) {
	let diffs:Vec<usize> = adapters.iter().zip(&adapters[1..]).map(|(a,b)| {
		b - a
	}).collect();

	// starting from (0, 1) because we count the last diff as a 3
	diffs.iter().fold((0, 1), | (ones, threes), d| {
		match d {
			1 => (ones + 1, threes),
			3 => (ones, threes + 1),
			_ => (ones, threes)
		}
	})
}
fn main() -> Result<(), std::io::Error> {
	let mut input = get_file_input(1)?;
	input = format!("0\n{}", input);
	
	let mut adapters:Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();
	adapters.sort();

	let distribution = find_distribution(&adapters);

	println!("{}", distribution.0 * distribution.1);
	println!("{}", find_all_adapter_ways(&adapters));
	Ok(())
}
