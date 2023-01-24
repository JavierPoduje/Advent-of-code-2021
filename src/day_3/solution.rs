use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let lines = parse();
    let part1 = part1(lines.clone());
    let part2 = part2(lines);
    (part1.to_string(), part2.to_string())
}

fn part2(input_lines: Vec<String>) -> i32 {
    let mut sum = 0;

    for lines in input_lines.chunks(3) {
        let mut sets = lines
            .iter()
            .map(|l| HashSet::from_iter(l.chars()))
            .collect::<Vec<HashSet<char>>>();
        let mut dup = sets.pop().unwrap();
        for set in sets {
            dup = set.intersection(&dup).copied().collect();
        }
        let ch = *dup.iter().next().unwrap();
        sum += match ch {
            'a'..='z' => (ch as u8) - b'a' + 1,
            'A'..='Z' => (ch as u8) - b'A' + 27,
            _ => panic!("bad input"),
        } as i32;
    }

    sum
}

fn part1(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in &lines {
        let len = line.len() / 2;
        let (left, right) = (&line[0..len], &line[len..]);
        let left: HashSet<char> = HashSet::from_iter(left.chars());
        let right: HashSet<char> = HashSet::from_iter(right.chars());
        let dup: HashSet<&char> = left.intersection(&right).collect();
        let ch = **dup.iter().next().unwrap();

        sum += match ch {
            'a'..='z' => (ch as u8) - b'a' + 1,
            'A'..='Z' => (ch as u8) - b'A' + 27,
            _ => panic!("bad input"),
        } as i32;
    }
    sum
}

fn parse() -> Vec<String> {
    read_one_per_line::<String>("./src/day_3/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect()
}
