use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type AnyError = Box<dyn std::error::Error>;

fn main() -> Result<(), AnyError> {
    let input = File::open("inputs/day1")?;

    let inputs: Vec<i64> = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect();

    println!("1-1: {}", first(&inputs));
    println!("1-2: {}", second(&inputs));

    Ok(())
}

fn first(inputs: &[i64]) -> i64 {
    inputs.iter().sum()
}

fn second(inputs: &[i64]) -> i64 {
    let mut sum = 0;
    let mut duplicates = HashSet::new();
    duplicates.insert(0);

    loop {
        for n in inputs {
            sum += n;
            if duplicates.contains(&sum) {
                return sum;
            }

            duplicates.insert(sum);
        }
    }
}
