extern crate utils;
use std::collections::HashMap;

use utils::utils::*;

trait  Instruction {
    fn run(&self, c: &mut Computer);
}
struct SetValue {
    addr : i64,
    value : i64
}

struct SetMask(Vec<(usize, Option<i64>)>);

impl Instruction for SetValue {
    fn run(&self, c: &mut Computer) {
        // part1
        let v = c.mask.iter()
            .filter(|(_bit, v)| v.is_some())
            .fold(self.value, |value, (bit, v)| {
                ((v.unwrap().wrapping_neg() ^ value) & (1 << bit)) ^ value
            });
        c.memory.insert(self.addr, v);

        // part2
        let mut addresses: Vec<i64> = Vec::new();
        get_floating_adresses(self.addr, &c.mask, 0, &mut addresses);
        for a in addresses {
            c.memory2.insert(a, self.value);
        }
    }
}

impl Instruction for SetMask {
    fn run(&self, c: &mut Computer) {
        c.mask = self.0.clone();
    }
}
#[derive(Debug)]
struct Computer {
    memory : HashMap<i64, i64>,
    memory2: HashMap<i64, i64>,
    mask : Vec<(usize, Option<i64>)>
}

fn set_one(value: i64, bit: usize) -> i64 {
    value | (1 <<bit)
}

fn set_zero(value: i64, bit: usize) -> i64 {
    value & (!(1 << bit))
}

fn get_floating_adresses(value : i64, mask : &Vec<(usize, Option<i64>)>, mask_idx: usize, addrs: &mut Vec<i64>) {
    if mask_idx == mask.len() {
        addrs.push(value);
    } else {
        let (bit, bm) = mask[mask_idx];
        match bm {
            Some(1) => { 
                get_floating_adresses(set_one(value, bit), mask, mask_idx + 1, addrs);
            },
            Some(0) => {
                get_floating_adresses(value, mask, mask_idx + 1, addrs);
            },
            None => {
                get_floating_adresses(set_zero(value, bit), mask, mask_idx + 1, addrs);
                get_floating_adresses(set_one(value, bit), mask, mask_idx + 1, addrs);
            },
            _ => panic!("Invalid mask value")
        }
    }
}

fn parse_mask(input: &str) -> Vec<(usize, Option<i64>)> {
    input
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if c.is_digit(10) {
                (i, Some(c.to_digit(10).unwrap() as i64 ))
            } else {
                (i, None)
            }

        })
        .collect()
}

fn parse(input : &str) -> Vec<Box<dyn Instruction>> {
    input
        .lines()
        .map(|l| -> Box< dyn Instruction> {
            let mut tokens = l.split(" = ");
            match tokens.next() {
                Some("mask") => {
                    Box::new(SetMask(parse_mask(tokens.next().unwrap())))
                },
                Some(mem) => {
                    if let Some(p) = mem.find("[") {
                        Box::new(SetValue{
                            addr : mem[p+1..mem.len()-1].parse::<i64>().unwrap(), 
                            value : tokens.next().unwrap().parse::<i64>().unwrap()
                        })
                    } else {
                        panic!("Invalid input");
                    }
                }
                None => panic!("Invalid input")
            }
            
        })
        .collect()

}

fn main() -> Result<(), std::io::Error>{
    let input = get_file_input(1)?;
    let instructions = parse(&input);
    let mut computer: Computer = Computer { memory: HashMap::new(), memory2: HashMap::new(), mask : vec![]};
    for instr in instructions {
        instr.run(&mut computer);
    }

    let p1 = computer.memory.iter().fold(0, |sum, (_key, value)| {
        sum + value
    });
    let p2 = computer.memory2.iter().fold(0, |sum, (_key, value)| {
        sum + value
    });

    println!("sum1 : {}", p1);
    println!("sum2 : {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn t42() {
        let mask: Vec<(usize, Option<i64>)> = parse_mask("000000000000000000000000000000X1001X");
        let expected:Vec<i64> = vec![26, 27, 58, 59];
        let mut addrs: Vec<i64> = Vec::new();
        get_floating_adresses(42, &mask, 0, &mut addrs);
        addrs.sort();
        assert_eq!(expected, addrs);
    }
}
