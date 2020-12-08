use std::fs;
use std::{collections::HashSet, env};

#[derive(Debug)]
enum Operation {
    ACC,
    JMP,
    NOP,
}
#[derive(Debug)]
struct Instruction {
    op: Operation,
    arg: i32,
}

struct Console {
    accumulator: i32,
    instruction_history: HashSet<usize>,
}

impl Console {
    fn clean_run(&mut self, instr: &Vec<Instruction>) -> Option<i32> {
        self.instruction_history.clear();
        self.accumulator = 0;
        let mut current_instruction: i32 = 0;
        loop {
            if current_instruction < 0 {
                println!("Invalid instruction pointer {}", current_instruction);
                return None;
            }

            if self.instruction_history.contains(&(current_instruction as usize)) {
                println!("This instruction set has a cycle(acc {})", self.accumulator);
                return None;
            }

            let executed_instruction = current_instruction;
            match instr[current_instruction as usize].op {
                Operation::ACC => {
                    self.accumulator += instr[current_instruction as usize].arg;
                    current_instruction += 1;
                }
                Operation::NOP => {
                    current_instruction += 1;
                }
                Operation::JMP => {
                    current_instruction += instr[current_instruction as usize].arg;
                }
            }

            self.instruction_history.insert(executed_instruction as usize);

            if current_instruction as usize >= instr.len() {
                break;
            }
        }

        Some(self.accumulator)
    }
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            let arg = parts[1].parse::<i32>().unwrap();
            match parts[0] {
                "acc" => Instruction {
                    op: Operation::ACC,
                    arg: arg,
                },
                "nop" => Instruction {
                    op: Operation::NOP,
                    arg: arg,
                },
                "jmp" => Instruction {
                    op: Operation::JMP,
                    arg: arg,
                },
                _ => panic!("Unsupported operation yet"),
            }
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

    let mut instructions = parse_instructions(&file_contents);
    let mut console: Console = Console {
        accumulator: 0,
        instruction_history: HashSet::new(),
    };
    println!("{:?}", console.clean_run(&instructions));

    for i in 0..instructions.len() {
        match instructions[i].op {
            Operation::NOP => {
                instructions[i].op = Operation::JMP;
                if let Some(acc) = console.clean_run(&instructions) {
                    println!("Acc : {}", acc);
                    break;
                } else {
                    instructions[i].op = Operation::NOP;
                }
            }
            Operation::JMP => {
                instructions[i].op = Operation::NOP;
                if let Some(acc) = console.clean_run(&instructions) {
                    println!("Acc : {}", acc);
                    break;
                } else {
                    instructions[i].op = Operation::JMP;
                }
            }
            _ => {}
        }
    }
}
