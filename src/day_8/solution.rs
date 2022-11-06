use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let rows: Vec<Row> = read_one_per_line::<String>("./src/day_8/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| Row::new(row))
        .collect();

    let mut part1_count = 0;
    let mut part2_count = 0;
    for row in rows {
        let hashmap = build_row_hashmap(&row.data);
        let mut output = row.output;
        output.reverse();

        let mut part2_num = 0;
        for (idx, str) in output.into_iter().enumerate() {
            let value = hashmap.get(&str).unwrap();
            match value {
                2 | 4 | 3 | 7 => {
                    part1_count += value;
                    part2_num += value * u32::pow(10, idx as u32);
                }
                _ => {
                    part2_num += value * u32::pow(10, idx as u32);
                }
            }
        }
        part2_count += part2_num;
    }

    (part1_count as u64, part2_count as u64)
}

fn build_row_hashmap(data: &Vec<String>) -> HashMap<String, u32> {
    let mut hashmap: HashMap<String, u32> = HashMap::new();

    // define 1, 4, 7 and 8
    for str in data {
        match str.len() {
            2 => {
                hashmap.insert(str.to_string(), 1);
            }
            3 => {
                hashmap.insert(str.to_string(), 7);
            }
            4 => {
                hashmap.insert(str.to_string(), 4);
            }
            7 => {
                hashmap.insert(str.to_string(), 8);
            }
            _ => (),
        }
    }

    // nine, zero and three
    for str in data.into_iter().filter(|value| value.len() == 6) {
        if contains(&str, &hashmap, &4) {
            hashmap.insert(str.to_string(), 9);
        } else if contains(&str, &hashmap, &7) {
            hashmap.insert(str.to_string(), 0);
        } else {
            hashmap.insert(str.to_string(), 6);
        }
    }
    // two, three and five
    for str in data.into_iter().filter(|value| value.len() == 5) {
        let six = key_of(&hashmap, &6);

        if contains(&str, &hashmap, &1) {
            hashmap.insert(str.to_string(), 3);
        } else if str.chars().all(|char| six.contains(char)) {
            hashmap.insert(str.to_string(), 5);
        } else {
            hashmap.insert(str.to_string(), 2);
        }
    }

    hashmap
}

fn contains(str: &str, hashmap: &HashMap<String, u32>, contained_number: &u32) -> bool {
    let candidate: Vec<char> = str.chars().collect();
    key_of(&hashmap, contained_number)
        .clone()
        .chars()
        .all(|char| candidate.contains(&char))
}

fn key_of(hashmap: &HashMap<String, u32>, target: &u32) -> String {
    for (key, val) in hashmap.iter() {
        if val == target {
            return key.to_string();
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct Row {
    data: Vec<String>,
    output: Vec<String>,
}

impl Row {
    pub fn new(raw_line: String) -> Self {
        let pairs = raw_line
            .split("|")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|pair| {
                pair.split(" ")
                    .filter(|value| !value.is_empty())
                    .collect::<Vec<&str>>()
            })
            .collect::<Vec<Vec<&str>>>();

        let mut data: Vec<Vec<String>> = Vec::new();
        let mut output: Vec<Vec<String>> = Vec::new();
        for (idx, pair) in pairs.into_iter().enumerate() {
            let response = pair
                .into_iter()
                .map(|value| {
                    let mut sorted = value.chars().collect::<Vec<char>>();
                    sorted.sort_by(|a, b| a.cmp(b));
                    String::from_iter(sorted)
                })
                .collect::<Vec<_>>();

            if idx == 0 {
                data.push(response);
            } else {
                output.push(response);
            }
        }

        Self {
            data: data[0].clone(),
            output: output[0].clone(),
        }
    }
}
