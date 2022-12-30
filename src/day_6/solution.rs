use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let line: String = read_one_per_line::<String>("./src/day_6/input.txt")
        .unwrap()
        .first()
        .unwrap()
        .to_string();

    let raw_chars = line.chars().collect::<Vec<char>>();

    let windowed_chars = raw_chars.windows(4).enumerate();

    let mut part_1: String = String::new();
    for (idx, chars) in windowed_chars {
        let unique_chars: HashSet<_> = chars.clone().iter().collect();
        if unique_chars.len() == chars.len() {
            part_1.push_str(&(idx + 4).to_string());
            break;
        }
    }

    (part_1, "A".to_string())
}
