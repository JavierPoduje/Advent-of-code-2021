use super::super::utils::read_one_per_line::read_one_per_line;
use std::cmp::min;

pub fn solution() -> (usize, usize) {
    let input = read_one_per_line::<String>("./src/day_7/input.txt").unwrap();
    let values: Vec<usize> = input
        .first()
        .unwrap()
        .split(",")
        .map(|value| value.parse::<usize>().unwrap())
        .collect();

    let crabs = crabs(&values);

    let part1 = calculate_fuel_to_use(&crabs, false);
    let part2 = calculate_fuel_to_use(&crabs, true);

    (part1, part2)
}

fn calculate_fuel_to_use(crabs: &Vec<usize>, penalize_distance: bool) -> usize {
    let mut used_fuel: Option<usize> = None;
    for to_idx in 0..crabs.len() {
        let mut current_fuel = 0;
        for (crab_idx, number_of_crabs) in crabs.iter().enumerate() {
            let mut diff = abs_diff(to_idx, crab_idx);

            if penalize_distance {
                diff = factorial(diff);
            }

            current_fuel += diff * number_of_crabs;
        }

        if let Some(fuel) = used_fuel {
            used_fuel = Some(min(fuel, current_fuel));
        } else {
            used_fuel = Some(current_fuel);
        }
    }
    used_fuel.unwrap()
}

fn crabs(values: &Vec<usize>) -> Vec<usize> {
    let max = values.iter().max().unwrap();
    let mut crabs = vec![0; (*max as usize) + 1];

    for idx in values {
        crabs[*idx] += 1;
    }

    crabs
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a == b {
        return 0;
    } else if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

fn factorial(num: usize) -> usize {
    match num {
        0 => 0,
        1 => 1,
        _ => num + factorial(num - 1),
    }
}
