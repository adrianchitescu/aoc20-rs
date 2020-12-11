extern crate utils;
use utils::utils::*;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
enum GridState {
    Empty,
    Occupied,
    Floor
}
#[derive(Debug)]
struct Layout {
    grid : Vec<Vec<GridState>>
}
impl fmt::Display for Layout {
    fn fmt(&self, fmt: &mut  fmt::Formatter) -> fmt::Result {
        for l in &self.grid {
            for s in l {
                let c = match s {
                    GridState::Empty => 'L',
                    GridState::Occupied => '#',
                    GridState::Floor => '.'
                };
                fmt.write_str(&c.to_string());
            }
            fmt.write_str("\n");
        }
        Ok(())
    }
}
impl Layout {
    fn str(&self) -> String {
        format!("{:?}", self.grid)
    }

    fn occupy(&self, level : usize, toleration : usize) -> Vec<Vec<GridState>> {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, seat)| {
                        if *seat == GridState::Floor {
                            GridState::Floor
                        } else {
                            let empty_arround = self.see_occupied(level, (x as i32, y as i32));
                            match empty_arround {
                                0 => GridState::Occupied,
                                _ if empty_arround >= toleration =>  GridState::Empty,
                                _ => *seat
                            }
                        }
                    }).collect()
            }).collect()
    }

    fn is_valid(&self, p : (i32, i32)) -> bool {
        p.0 >= 0 && p.0 < self.grid[0].len() as i32 &&
        p.1 >= 0 && p.1 < self.grid.len() as i32
    }

    fn see_occupied(&self, level: usize, p : (i32, i32))-> usize {
        let directions = vec![(-1,-1), (-1, 0), (-1, 1), (0,-1), (0, 1), (1,-1), (1, 0), (1, 1)];
        directions.iter().fold(0, |occ, (dx, dy)| {
            let mut px = p.0;
            let mut py = p.1;
            let mut found = 0;
            for _ in 0..level {
                px += dx;
                py += dy;
                if !self.is_valid((px, py)) {
                    break;
                } else {
                    if self.grid[py as usize][px as usize] == GridState::Occupied {
                        found = 1;
                        break;
                    } else if self.grid[py as usize][px as usize] == GridState::Empty {
                        break;
                    }
                }
            }
            occ + found
        })
    }

    fn count_occupied(&self) -> usize {
        self.grid.iter().fold(0, |cnt, line| 
            cnt + line.iter().filter(|s| **s == GridState::Occupied).count()
        )
    }

    fn occupy_loop(&mut self, level:usize, tolerance: usize) -> usize {
        loop {
            // println!("{}", self);
            let new_layout = self.occupy(level, tolerance);
            if self.str() == format!("{:?}", new_layout) {
                break;
            }
            self.grid = new_layout;
        }
    
        self.count_occupied()
    }
}

fn parse_input(input: &str) -> Vec<Vec<GridState>> {
    input
        .lines()
        .map(|l| {
            l
                .chars()
                .map(|c| {
                    match c {
                        'L' => GridState::Empty,
                        '.' => GridState::Floor,
                        '#' => GridState::Occupied,
                        _ => panic!("Invalid input")
                    }
                })
                .collect::<Vec<GridState>>()
        }).collect()
}


fn main() -> Result<(), std::io::Error> {
    let input = get_file_input(1)?;
    let mut seat_layout = Layout { grid : parse_input(&input)};
    let mut seat_layout2 = Layout { grid : seat_layout.grid.clone() };

    println!("Stabilizaation. {} occupied", seat_layout.occupy_loop(1, 4));
    println!("Stabilizaation. {} occupied", seat_layout2.occupy_loop(seat_layout2.grid.len(), 5));
    Ok(())
}
