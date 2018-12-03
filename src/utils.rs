use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn get_inputs<T: FromStr>(day_number: u8) -> Vec<T> {
    let file_path = format!("inputs/day{}", day_number);
    let input = File::open(&file_path)
        .unwrap_or_else(|_| panic!(format!("Error opening file {}", &file_path)));
    let lines: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();

    lines
        .iter()
        .map(|l| match T::from_str(l) {
            Ok(v) => v,
            Err(e) => panic!(),
        })
        .collect()
}
