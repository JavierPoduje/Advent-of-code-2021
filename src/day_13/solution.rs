use std::collections::{HashSet, VecDeque};

use super::super::utils::read_one_per_line::read_one_per_line;

type Coord = (usize, usize);
type Fold = (String, usize);

pub fn solution() -> (u64, u64) {
    let mut input = read_one_per_line::<String>("./src/day_13/input.txt").unwrap();
    let mut coords: HashSet<Coord> = get_coords(&input);
    let folds: VecDeque<Fold> = get_folds(&mut input);

    let rows = coords.clone().into_iter().fold(1, |acc, (_, y)| {
        if y >= acc {
            return y + 1;
        }
        return acc;
    });
    let cols = coords.clone().into_iter().fold(1, |acc, (x, _)| {
        if x >= acc {
            return x + 1;
        }
        return acc;
    });

    let mut part1 = 0;
    for (i, (axis, idx)) in folds.into_iter().enumerate() {
        if axis == "y" {
            horizontal_fold(idx, &mut coords, rows);
        } else if axis == "x" {
            vertical_fold(idx, &mut coords, cols);
        }

        if i == 0 {
            part1 = coords.len();
        }
    }

    print(&coords);

    let part2 = coords.len();
    (part1 as u64, part2 as u64)
}

fn print(coords: &HashSet<Coord>) {
    let rows = coords.clone().into_iter().fold(1, |acc, (_, y)| {
        if y >= acc {
            return y + 1;
        }
        return acc;
    });
    let cols = coords.clone().into_iter().fold(1, |acc, (x, _)| {
        if x >= acc {
            return x + 1;
        }
        return acc;
    });

    for row in 0..rows {
        for col in 0..cols {
            if coords.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn horizontal_fold(idx: usize, coords: &mut HashSet<Coord>, rows: usize) {
    for (x, y) in coords.clone().into_iter() {
        if y >= idx {
            coords.remove(&(x, y));
            coords.insert((x, rows - 1 - y));
        }
    }
}

fn vertical_fold(idx: usize, coords: &mut HashSet<Coord>, cols: usize) {
    for (x, y) in coords.clone().into_iter() {
        if x >= idx {
            coords.remove(&(x, y));
            coords.insert((cols - 1 - x, y));
        }
    }
}

fn get_coords(input: &Vec<String>) -> HashSet<Coord> {
    let mut coords: HashSet<Coord> = HashSet::new();

    for row in input.into_iter() {
        if row == "" {
            break;
        }

        let raw_coord: Vec<usize> = row
            .split(",")
            .map(|value| value.parse::<usize>().unwrap())
            .collect();
        coords.insert((
            raw_coord.first().unwrap().clone(),
            raw_coord.last().unwrap().clone(),
        ));
    }

    coords
}

fn get_folds(input: &mut Vec<String>) -> VecDeque<Fold> {
    let mut folds: VecDeque<Fold> = VecDeque::new();
    input.reverse();

    for (idx, row) in input.into_iter().enumerate() {
        if idx == 0 && row == "" {
            continue;
        } else if row == "" {
            break;
        }

        let raw_fold = row.split_whitespace().last().unwrap();
        let axis = raw_fold.chars().nth(0).unwrap().to_string();
        let idx = raw_fold
            .chars()
            .nth(2)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        folds.push_front((axis, idx));
    }

    folds
}
