use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct ElfClaim {
    pub id: u64,
    pub left: u64,
    pub top: u64,
    pub width: u64,
    pub height: u64,
}

impl ElfClaim {
    pub fn points(&self) -> Vec<(u64, u64)> {
        let mut result = Vec::new();

        let right = self.left + self.width;
        let bottom = self.top + self.height;

        for x in self.left..right {
            for y in self.top..bottom {
                result.push((x, y));
            }
        }

        result
    }
}

impl FromStr for ElfClaim {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        let claim = ElfClaim {
            id: captures[1].parse::<u64>()?,
            left: captures[2].parse::<u64>()?,
            top: captures[3].parse::<u64>()?,
            width: captures[4].parse::<u64>()?,
            height: captures[5].parse::<u64>()?,
        };

        Ok(claim)
    }
}

pub fn day3_1() {
    let inputs: Vec<ElfClaim> = crate::utils::get_inputs(3);

    let mut counts: HashMap<(u64, u64), u64> = HashMap::new();

    for input in inputs {
        for point in input.points() {
            let count = counts.entry(point).or_insert(0);
            *count += 1;
        }
    }

    let answer = counts.values().filter(|c| **c > 1).count();

    println!("3-1: {}", answer);
}

pub fn day3_2() {
    let inputs: Vec<ElfClaim> = crate::utils::get_inputs(3);

    let mut possibilities: HashSet<u64> = inputs.iter().map(|i| i.id).collect();
    let mut occupied: HashMap<(u64, u64), u64> = HashMap::new();

    for input in inputs {
        for point in input.points() {
            if occupied.contains_key(&point) {
                let occupant = occupied[&point];
                possibilities.remove(&input.id);
                possibilities.remove(&occupant);
            }
            occupied.insert(point, input.id);
        }
    }

    if possibilities.len() != 1 {
        println!("{:?}", possibilities);
        panic!(format!(
            "{} possibilites remaining. Should be 1.",
            possibilities.len()
        ));
    }

    let answer = possibilities.iter().nth(0).unwrap();

    println!("3-2: {}", answer);
}

fn draw(inputs: &[ElfClaim]) {
    let mut occupied: HashMap<(u64, u64), Option<u64>> = HashMap::new();

    let mut x_range = (0, 0);
    let mut y_range = (0, 0);

    for input in inputs {
        for point in input.points() {
            x_range.0 = point.0.min(x_range.0);
            x_range.1 = point.0.max(x_range.1);

            y_range.0 = point.1.min(y_range.0);
            y_range.1 = point.1.max(y_range.1);

            if occupied.contains_key(&point) {
                occupied.insert(point, None);
            } else {
                occupied.insert(point, Some(input.id));
            }
        }
    }

    for y in y_range.0..=y_range.1 + 1 {
        for x in x_range.0..=x_range.1 + 1 {
            if occupied.contains_key(&(x, y)) {
                match occupied[&(x, y)] {
                    Some(index) => print!("{}", index),
                    None => print!("X"),
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
