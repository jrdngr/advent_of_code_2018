use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

use crate::Result;

pub fn day5_1() -> Result<()> {
    let input = get_input()?;
    let mut polymer = Polymer::from_str(&input)?;
    polymer.reduce();

    println!("4-1: {}", polymer.len());

    Ok(())
}

pub fn day5_2() {}

#[derive(Debug)]
struct Polymer {
    units: Vec<u32>,
}

impl FromStr for Polymer {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let units: Vec<u32> = s.chars().map(u32::from).collect();
        Ok(Polymer { units })
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let polymer_string: String = self
            .units
            .iter()
            .map(|c| std::char::from_u32(*c).unwrap())
            .collect();
        write!(f, "{}", polymer_string)
    }
}

impl Polymer {
    pub fn len(&self) -> usize {
        self.units.len()
    }

    pub fn reduce(&mut self) {
        while self.reduction_pass() > 0 {
            continue;
        }
    }

    fn reduction_pass(&mut self) -> u64 {
        if self.units.len() < 2 {
            return 0;
        }

        let mut result: Vec<Option<u32>> = self.units.iter().map(|u| Some(*u)).collect();

        let mut cursor1 = 0;
        let mut cursor2 = 1;
        let mut annihilation_count = 0;

        while cursor2 < result.len() {
            if let Some(v1) = result[cursor1] {
                if let Some(v2) = result[cursor2] {
                    if Polymer::should_annihilate(v1, v2) {
                        annihilation_count += 1;
                        result[cursor1] = None;
                        result[cursor2] = None;
                        cursor1 += 2;
                        cursor2 += 2;
                    } else {
                        cursor1 += 1;
                        cursor2 += 1;
                    }
                }
            }
        }

        self.units = result
            .iter()
            .filter(|u| u.is_some())
            .map(|u| u.unwrap())
            .collect();

        annihilation_count
    }

    fn should_annihilate(n1: u32, n2: u32) -> bool {
        let max = n1.max(n2);
        let min = n1.min(n2);

        max - min == 32
    }
}

fn get_input() -> Result<String> {
    let path = "inputs/day5";
    let input_file =
        File::open(path).unwrap_or_else(|_| panic!(format!("Error opening file {}", path)));

    let mut input = String::new();
    BufReader::new(input_file).read_to_string(&mut input)?;
    if input.ends_with('\n') {
        input.pop();
    }

    Ok(input.to_string())
}
