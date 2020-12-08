use std::env;
use std::fs;
use std::collections::HashMap;
#[derive(Debug)]
struct Rules(HashMap<String, Vec<(usize, String)>>);

fn parse_input(input: &String) -> Rules {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .fold(Rules(HashMap::new()), |mut r, line| {
            let parts:Vec<&str> = line[0..line.len() -1].split(" contain ").collect();
            match parts[1] {
                "no other bags" => {
                    r.0.insert(parts[0].to_string(), vec![]);
                },
                _ => {
                    let contain:Vec<(usize, String)> = parts[1].split(',')
                            .map(|c| {
                                let words:Vec<&str> = c.trim().split(' ').collect();
                                let bags_no:usize = words[0].parse::<usize>().unwrap_or(0);
                                let bag_name = format!("{} {} bags", words[1], words[2]);
                                (bags_no, bag_name)
                            })
                            .collect();
                    r.0.insert(parts[0].to_string(), contain);
                }
            }
            r
        })
}

fn can_hold(rules: &Rules, who: &str, what: &str) -> bool{
    match rules.0.get(who) {
        Some(h) => {
            h.iter().any(|(_, bag)|
                bag == what || can_hold(rules, bag, what)
            )
        }
        None => false
    }
}

fn count_all(rules: &Rules, bag: &str) -> usize {
    match rules.0.get(bag) {
        None => 0,
        Some(holds) => {
            holds.iter().fold(0, |s, (count, b)| {
                s + count * (1 + count_all(rules, b))
            })
        }
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

    let rules = parse_input(&file_contents);
    let can_hold_shiny = rules.0
        .keys()
        .filter(|k| *k != "shiny gold bags")
        .fold(0, |s, bag| {
            s + can_hold(&rules, bag, "shiny gold bags") as usize
        });
    
    println!("can hold shiny {}", can_hold_shiny);
    println!("shiny bag must have {} bags", count_all(&rules, &"shiny gold bags"));
}