use std::collections::HashSet;

pub fn day1_1() {
    let inputs: Vec<i64> = crate::utils::get_inputs(1);
    let sum: i64 = inputs.iter().sum();

    println!("1-1: {}", sum);
}

pub fn day1_2() {
    let inputs: Vec<i64> = crate::utils::get_inputs(1);

    let mut sum = 0;
    let mut duplicates = HashSet::new();
    duplicates.insert(0);

    loop {
        for n in &inputs {
            sum += n;
            if duplicates.contains(&sum) {
                println!("1-2: {}", sum);
                return;
            }

            duplicates.insert(sum);
        }
    }
}
