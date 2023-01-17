use std::collections::{HashSet, VecDeque};

use super::super::utils::read_one_per_line::read_one_per_line;

const DELTA_XYZ: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

pub fn solution() -> (String, String) {
    let cubes = parse();
    let part1 = part1(cubes.clone());
    let part2 = part2(cubes);
    (part1.to_string(), part2.to_string())
}

fn part1(cubes: HashSet<(i32, i32, i32)>) -> usize {
    let mut sides = cubes.len() * 6;

    for c in &cubes {
        for d in DELTA_XYZ {
            let pos = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
            if cubes.contains(&pos) {
                sides -= 1;
            }
        }
    }

    sides
}

fn part2(cubes: HashSet<(i32, i32, i32)>) -> i32 {
    let mut xrange = (i32::MAX, i32::MIN);
    let mut yrange = (i32::MAX, i32::MIN);
    let mut zrange = (i32::MAX, i32::MIN);

    for c in &cubes {
        xrange.0 = xrange.0.min(c.0);
        xrange.1 = xrange.1.max(c.0);

        yrange.0 = yrange.0.min(c.1);
        yrange.1 = yrange.1.max(c.1);

        zrange.0 = zrange.0.min(c.2);
        zrange.1 = zrange.1.max(c.2);
    }

    xrange = (xrange.0 - 1, xrange.1 + 1);
    yrange = (yrange.0 - 1, yrange.1 + 1);
    zrange = (zrange.0 - 1, zrange.1 + 1);

    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back((xrange.0, yrange.0, zrange.0));

    let mut count = 0;
    while let Some(pos) = to_visit.pop_front() {
        if !seen.insert(pos) {
            continue;
        }

        for d in DELTA_XYZ {
            let next_pos = (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2);

            if next_pos.0 < xrange.0
                || next_pos.0 > xrange.1
                || next_pos.1 < yrange.0
                || next_pos.1 > yrange.1
                || next_pos.2 < zrange.0
                || next_pos.2 > zrange.1
            {
                continue;
            }

            if cubes.contains(&next_pos) {
                count += 1;
            } else {
                to_visit.push_back(next_pos);
            }
        }
    }

    count
}

fn parse() -> HashSet<(i32, i32, i32)> {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_18/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let mut cubes = HashSet::new();
    for line in lines {
        let formatted: Vec<i32> = line.split(",").map(|v| v.parse::<i32>().unwrap()).collect();
        cubes.insert((formatted[0], formatted[1], formatted[2]));
    }

    cubes
}
