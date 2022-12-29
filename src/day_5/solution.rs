use std::collections::{HashMap, VecDeque};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let rows: Vec<String> = read_one_per_line::<String>("./src/day_5/input.txt")
        .unwrap()
        .into_iter()
        .collect();

    let stack_definition: Vec<_> = rows.iter().take_while(|row| !row.is_empty()).collect();
    let rearragements: Vec<Movement> =
        rows.iter()
            .filter(|row| row.starts_with("move"))
            .fold(Vec::new(), |mut acc, row| {
                let words: Vec<&str> = row.split(" ").collect();
                let repetitions = words[1].parse::<i32>().unwrap();
                let from = words[3].parse::<usize>().unwrap();
                let to = words[5].parse::<usize>().unwrap();
                acc.push(Movement::new(repetitions, from, to));
                acc
            });
    let mut stacks_by_idx = build_initial_stacks(stack_definition);

    for mov in rearragements {
        for _ in 0..mov.repetitions {
            let item = stacks_by_idx
                .get_mut(&mov.from)
                .unwrap()
                .pop_back()
                .unwrap();
            stacks_by_idx.get_mut(&mov.to).unwrap().push_back(item);
        }
    }

    let mut part_1 = String::new();
    for idx in 1..=stacks_by_idx.len() {
        part_1.push_str(stacks_by_idx.get(&idx).unwrap().back().unwrap());
    }

    (part_1, 0.to_string())
}

fn build_initial_stacks(rows: Vec<&String>) -> HashMap<usize, VecDeque<String>> {
    let mut parsed_rows = rows
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<Vec<Vec<char>>>, row| {
            let mut stacks: Vec<Vec<char>> = Vec::new();
            let mut stack: Vec<char> = Vec::new();

            for row_char in row.chars() {
                if stack.len() == 4 {
                    stacks.push(stack);
                    stack = Vec::new();
                }
                stack.push(row_char);
            }
            stacks.push(stack);

            acc.push(stacks);
            acc
        });

    let mut hashes = parsed_rows.last().unwrap().iter().fold(
        HashMap::<usize, VecDeque<String>>::new(),
        |mut acc, item| {
            let joined = item.iter().collect::<String>();
            acc.insert(
                joined.clone().trim().parse::<usize>().unwrap(),
                VecDeque::new(),
            );
            acc
        },
    );

    parsed_rows.reverse();

    for parsed_row in parsed_rows.iter().skip(1) {
        for (idx, raw_item) in parsed_row.iter().enumerate() {
            let hash = hashes.get_mut(&(idx + 1)).unwrap();
            let joined = raw_item.iter().collect::<String>();
            let trimmed = joined.trim().replace("[", "").replace("]", "");
            if !trimmed.is_empty() {
                hash.push_back(trimmed.to_string().to_owned());
            }
        }
    }

    hashes
}

#[derive(Debug)]
struct Movement {
    repetitions: i32,
    from: usize,
    to: usize,
}

impl Movement {
    pub fn new(repetitions: i32, from: usize, to: usize) -> Self {
        Self {
            repetitions,
            from,
            to,
        }
    }
}
