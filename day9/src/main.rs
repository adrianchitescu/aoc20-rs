extern crate utils;
use std::collections::HashSet;

use utils::utils::*;

fn is_2sum(arr: &[i64], sum:i64) -> bool {
    let mut h:HashSet<i64> = HashSet::with_capacity(arr.len());
    for n in arr {
        if h.contains(n) {
            return true;
        } else {
            h.insert(sum -n);
        }
    }

    false
}

fn get_invalid_number(numbers: &[i64], preamble: usize) -> i64 {
    let mut invalid_number = 0;
    for i in preamble..numbers.len()- preamble - 1  {
        if !is_2sum(&numbers[i-preamble..i], numbers[i]){
            invalid_number = numbers[i];
            break;
        }
    }
    invalid_number
}

fn get_sum_set(numbers: &[i64], sum : i64) -> Option<&[i64]> {
    let mut s = numbers[0];
    let mut left =  0;
    let mut right = 0;
    loop {
        if s == sum {
            return Some(&numbers[left..=right]);
        } else if s < sum {
            right += 1;
            if right == numbers.len() {
                break;
            }
            s += numbers[right];
        } else {
            left += 1;
            s -= numbers[left-1];
        }
    }

    None
}

fn sum_min_max(no: &[i64]) -> i64{
    no.iter().min().unwrap() + no.iter().max().unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let input = get_file_input(1)?;
    // print
    let numbers:Vec<i64> = input.lines().map(|l| l.parse::<i64>().unwrap() ).collect();
    let invalid_no = get_invalid_number(&numbers, 25);
    println!("{} is the first invalid no", invalid_no);

    if let Some(sum_set) = get_sum_set(&numbers, invalid_no) {
        println!("{:?} ", sum_min_max(sum_set));
    } else {
        println!("Failed to find such a set");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_sum_set, sum_min_max};
    #[test]
    fn min_max_sum() {
        let no = vec![35, 20 ,15 ,25 ,47 ,40 ,62 ,55 ,65 ,95 ,102 ,117 ,150 ,182 ,127 ,219 ,299 ,277 ,309 ,576];
        assert_eq!(sum_min_max(get_sum_set(&no, 127).unwrap()), 62);
    }
}
