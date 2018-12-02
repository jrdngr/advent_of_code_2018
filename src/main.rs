use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type AnyError = Box<dyn std::error::Error>;

fn main() -> Result<(), AnyError> {
    println!("2-1: {}", day2_1());
    println!("2-2: {}", day2_2());

    Ok(())
}

fn day2_1() -> i64 {
    let input = File::open("inputs/day2").expect("Error opening file");
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    let mut doubles = 0;
    let mut triples = 0;

    let mut counts: HashMap<char, u64> = HashMap::new();
    for line in lines {
        for c in line.chars() {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
        }

        let mut double = false;
        let mut triple = false;
        for count in counts.values() {
            if *count == 2 {
                double = true;
            } else if *count == 3 {
                triple = true;
            }
        }

        if double {
            doubles += 1;
        }
        if triple {
            triples += 1;
        }

        counts.clear();
    }

    doubles * triples
}

fn day2_2() -> String {
    let input = File::open("inputs/day2").expect("Error opening file");
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    for i in 0..lines.len() {
        let current: Vec<char> = lines[i].chars().collect();
        'line: for j in i..lines.len() {
            let other: Vec<char> = lines[j].chars().collect();

            let mut difference_index: Option<usize> = None;
            for k in 0..current.len() {
                let first = current[k];
                let second = other[k];
                if first != second {
                    if difference_index.is_some() {
                        continue 'line;
                    }
                    difference_index = Some(k);
                }
            }

            if let Some(index) = difference_index {
                let mut result = current.clone();
                result.remove(index);
                return result.into_iter().collect();
            }
        }
    }

    "hi".to_string()
}

fn day1_1() -> i64 {
    let input = File::open("inputs/day1").expect("Error opening file");
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    lines.iter().map(|l| l.parse::<i64>().unwrap()).sum()
}

fn day1_2() -> i64 {
    let input = File::open("inputs/day1").expect("Error opening file");
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    let inputs: Vec<i64> = lines.iter().map(|l| l.parse::<i64>().unwrap()).collect();

    let mut sum = 0;
    let mut duplicates = HashSet::new();
    duplicates.insert(0);

    loop {
        for n in &inputs {
            sum += n;
            if duplicates.contains(&sum) {
                return sum;
            }

            duplicates.insert(sum);
        }
    }
}
