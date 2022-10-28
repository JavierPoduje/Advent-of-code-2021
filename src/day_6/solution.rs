use super::super::utils::read_one_per_line::read_one_per_line;
use std::collections::VecDeque;

pub fn solution() -> (u64, u64) {
    let input = read_one_per_line::<String>("./src/day_6/input.txt").unwrap();
    let values: Vec<u32> = input
        .first()
        .unwrap()
        .split(",")
        .map(|value| value.parse::<u32>().unwrap())
        .collect();

    let fishes = get_fishes(values);

    let part1 = count(fishes.clone(), 80);
    let part2 = count(fishes, 256);

    (part1, part2)
}

fn count(mut fishes: VecDeque<u64>, days: u32) -> u64 {
    for _ in 0..days {
        let babies = fishes.pop_front().unwrap();
        fishes[6] += babies;
        fishes.push_back(babies)
    }
    fishes.iter().sum()
}

fn get_fishes(values: Vec<u32>) -> VecDeque<u64> {
    let mut fishes = VecDeque::from(vec![0; 9]);

    for value in values {
        fishes[value as usize] += 1;
    }

    fishes
}
