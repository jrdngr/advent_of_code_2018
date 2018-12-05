use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, Read};
use std::rc::Rc;
use std::str::FromStr;

use crate::Result;

pub fn day5_1() -> Result<()> {
    let input = get_input()?;
    let mut polymer = Polymer::from_str(&input)?;
    polymer.reduce();

    println!("{}", polymer);

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
    pub fn reduce(&mut self) {}

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

    Ok(TEST_INPUT.to_string())
    //Ok(input.to_string())
}

const TEST_INPUT: &str = "aAcCbBEgfdgDFggdtJjrRrrOhhHHJhDdJkALJjjKkKHfJjlkrGggG";
