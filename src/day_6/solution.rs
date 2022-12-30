use std::{collections::HashSet, iter::Enumerate, slice::Windows};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let line: String = read_one_per_line::<String>("./src/day_6/input.txt")
        .unwrap()
        .first()
        .unwrap()
        .to_string();

    let raw_chars = line.chars().collect::<Vec<char>>();

    let fst_window = 4;
    let scd_window = 14;

    let part_1 = first_uniq_window(raw_chars.windows(fst_window).enumerate(), fst_window);
    let part_2 = first_uniq_window(raw_chars.windows(scd_window).enumerate(), scd_window);

    (part_1, part_2)
}

pub fn first_uniq_window(chars: Enumerate<Windows<char>>, window: usize) -> String {
    for (idx, chars) in chars {
        let unique_chars: HashSet<_> = chars.clone().iter().collect();
        if unique_chars.len() == chars.len() {
            return (idx + window).to_string();
        }
    }
    unreachable!()
}
