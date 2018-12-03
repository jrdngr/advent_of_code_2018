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

    for input in inputs.iter() {
        for point in input.points() {
            if occupied.contains_key(&point) {
                let occupant = occupied[&point];
                possibilities.remove(&input.id);
                possibilities.remove(&occupant);
                break;
            }
            occupied.insert(point, input.id);
        }
    }

    if possibilities.len() != 1 {
        println!("{:?}", possibilities);
        panic!("More than one possibility remaining");
    }

    let answer = possibilities.iter().nth(0).unwrap();

    println!("3-3: {}", answer);
}
