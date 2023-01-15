use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let rows: Vec<String> = read_one_per_line::<String>("./src/day_3/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let mut part1 = 0;
    for row in rows {
        let vec_row: Vec<&str> = row.split("").filter(|value| value != &"").collect();
        let chunks: Vec<Vec<&str>> = vec_row
            .chunks(row.len() / 2)
            .map(|chunk| chunk.to_vec())
            .collect();

        let fst = chunks.clone().first().unwrap().to_owned();
        let scd = chunks.clone().last().unwrap().to_owned();

        let left: HashSet<&str> = HashSet::from_iter(fst);
        let right: HashSet<&str> = HashSet::from_iter(scd);

        let intersections: u64 = left
            .clone()
            .intersection(&right)
            .map(|char| prioritize(char))
            .sum();

        part1 += intersections;
    }

    (part1.to_string(), 0.to_string())
}

fn prioritize(char: &str) -> u64 {
    match char {
        "a" => 1,
        "b" => 2,
        "c" => 3,
        "d" => 4,
        "e" => 5,
        "f" => 6,
        "g" => 7,
        "h" => 8,
        "i" => 9,
        "j" => 10,
        "k" => 11,
        "l" => 12,
        "m" => 13,
        "n" => 14,
        "o" => 15,
        "p" => 16,
        "q" => 17,
        "r" => 18,
        "s" => 19,
        "t" => 20,
        "u" => 21,
        "v" => 22,
        "w" => 23,
        "x" => 24,
        "y" => 25,
        "z" => 26,
        "A" => 27,
        "B" => 28,
        "C" => 29,
        "D" => 30,
        "E" => 31,
        "F" => 32,
        "G" => 33,
        "H" => 34,
        "I" => 35,
        "J" => 36,
        "K" => 37,
        "L" => 38,
        "M" => 39,
        "N" => 40,
        "O" => 41,
        "P" => 42,
        "Q" => 43,
        "R" => 44,
        "S" => 45,
        "T" => 46,
        "U" => 47,
        "V" => 48,
        "W" => 49,
        "X" => 50,
        "Y" => 51,
        "Z" => 52,
        _ => unreachable!(),
    }
}
