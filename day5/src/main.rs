use std::env;
use std::fs;
use itertools::Itertools;

fn decode(code: &str, range:(usize, usize), lower_half : char, upper_half:char) -> usize {
	code
		.chars()
		.fold(range, |(l,h), c| {
			let mid = l + (((h-l)/2) as usize);
			match c {
				_ if c == lower_half => (l, mid),
				_ if c == upper_half => (mid + 1, h),
				_ => (l, h)
			}
		})
		.0
}

fn get_seat_id(encoding: &str) -> usize {
	let row = decode(&encoding[0..7], (0, 127), 'F', 'B');
	let col = decode(&encoding[7..], (0, 7), 'L', 'R');

	row * 8 + col
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

	let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
	});
	
	let mut seats:Vec<usize> = file_contents
		.lines()
		.map(|l| get_seat_id(&l))
		.collect();
	

	seats.sort();
	let your_seat = match seats.iter().tuple_windows().find(|(l, h)| *h - *l > 1) {
		Some((l, h)) => (l + h) / 2,
		None => 0
	};

	println!("Max seat is : {:?}", seats.last());
	println!("Your seat is {}", your_seat);
}

#[cfg(test)]
mod tests {
	use crate::{ decode, get_seat_id };
	#[test]
	fn decode_test() {
		assert_eq!(decode(&"FBFBBFF", (0, 127), 'F', 'B'), 44);
		assert_eq!(decode(&"RLR", (0, 7), 'L', 'R'), 5);
	}

	#[test]
	fn seat_id() {
		assert_eq!(get_seat_id(&"BFFFBBFRRR".to_string()), 567);
		assert_eq!(get_seat_id(&"FFFBBBFRRR".to_string()), 119);
		assert_eq!(get_seat_id(&"BBFFBBFRLL".to_string()), 820);
	}
}
