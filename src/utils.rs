use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn get_inputs<T: FromStr>(day_number: u8) -> Vec<T> {
    get_inputs_from_path(&format!("inputs/day{}", day_number))
}

pub fn get_test_inputs<T: FromStr>(day_number: u8) -> Vec<T> {
    get_inputs_from_path(&format!("inputs/day{}test", day_number))
}

fn get_inputs_from_path<T: FromStr>(path: &str) -> Vec<T> {
    let input = File::open(path).unwrap_or_else(|_| panic!(format!("Error opening file {}", path)));
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    lines
        .iter()
        .map(|l| match T::from_str(l) {
            Ok(v) => v,
            Err(e) => panic!(),
        })
        .collect()
}
