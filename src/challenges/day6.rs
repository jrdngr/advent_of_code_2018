use std::collections::HashMap;

use crate::Result;

pub fn day6_1() -> Result<()> {
    let mut areas: AreaMap = get_inputs().iter().map(|i| (*i, 0)).collect();

    let (x_min, x_max, y_min, y_max) = get_boundaries(&areas);

    let width = x_max - x_min;
    let height = y_max - y_min;

    let mut grid: Grid = Vec::new();

    for y in 0..height {
        let mut row = Vec::new();
        for x in 0..width {
            row.push(((x, y), GridCell::Unchecked));
        }
        grid.push(row);
    }

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = (cell.0, cell_value(cell.0, &areas));
        }
    }

    for row in grid {
        for cell in row {
            if let (coordinates, GridCell::Single(x, y)) = cell {
                if coordinates.0 > x_min && coordinates.0 < x_max && coordinates.1 > y_min
                    || coordinates.1 < y_max
                {
                    let entry = areas.entry((x, y)).or_insert(0);
                    *entry += 1;
                }
            }
        }
    }

    let answer = areas.values().max().unwrap();

    println!("6_1: {}", answer);

    Ok(())
}

pub fn day6_2() -> Result<()> {
    println!("6_2: {}", 0);

    Ok(())
}

type Grid = Vec<Vec<((i16, i16), GridCell)>>;
type AreaMap = HashMap<(i16, i16), u64>;

#[derive(Debug, Clone)]
enum GridCell {
    Single(i16, i16),
    Multiple,
    Unchecked,
}

fn get_inputs() -> Vec<(i16, i16)> {
    let inputs: Vec<String> = crate::utils::get_test_inputs(6);

    inputs
        .iter()
        .map(|i| {
            let split: Vec<&str> = i.split(", ").collect();
            (
                split[0].parse::<i16>().unwrap(),
                split[1].parse::<i16>().unwrap(),
            )
        })
        .collect()
}

fn get_boundaries(inputs: &AreaMap) -> (i16, i16, i16, i16) {
    inputs.keys().fold((0, 0, 0, 0), |acc, input| {
        let x_min = acc.0.min(input.0);
        let x_max = acc.1.max(input.0);

        let y_min = acc.2.min(input.1);
        let y_max = acc.3.max(input.1);

        (x_min, x_max, y_min, y_max)
    })
}

fn distance(p1: (i16, i16), p2: (i16, i16)) -> u64 {
    ((p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()) as u64
}

fn cell_value(point: (i16, i16), areas: &AreaMap) -> GridCell {
    let mut min = std::u64::MAX;
    let mut next_min = std::u64::MAX;
    let mut closest_point = (0, 0);

    for target in areas.keys() {
        let d = distance(point, *target);
        if d < min {
            min = d;
            closest_point = *target;
        } else if d < next_min {
            next_min = d;
        }
    }

    if min == next_min {
        GridCell::Multiple
    } else {
        GridCell::Single(closest_point.0, closest_point.1)
    }
}
