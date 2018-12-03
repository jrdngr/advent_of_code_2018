use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day2_1() {
    let inputs: Vec<String> = crate::utils::get_inputs(2);

    let mut doubles = 0;
    let mut triples = 0;

    let mut counts: HashMap<char, u64> = HashMap::new();
    for line in inputs {
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
            if double && triple {
                break;
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

    let answer = doubles * triples;

    println!("2-1: {}", answer);
}

pub fn day2_2() {
    let inputs: Vec<String> = crate::utils::get_inputs(2);

    for i in 0..inputs.len() {
        let current: Vec<char> = inputs[i].chars().collect();
        'line: for j in i..inputs.len() {
            let other: Vec<char> = inputs[j].chars().collect();

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

                let answer: String = result.into_iter().collect();
                println!("2-2: {}", answer);
                return;
            }
        }
    }

    panic!("Could not find answer");
}
