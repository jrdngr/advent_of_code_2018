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
}

pub fn day3_2() {}
