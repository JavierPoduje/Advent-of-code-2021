use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let cubes = parse();
    let part1 = part1(cubes);
    (part1.to_string(), "B".to_string())
}

fn part1(cubes: HashSet<(i32, i32, i32)>) -> usize {
    let mut sides = cubes.len() * 6;
    let delta_xyz = [(-1, 0, 0), (1,0,0), (0,-1,0), (0,1,0), (0,0,-1), (0,0,1)];

    for c in &cubes {
        for d in delta_xyz {
            let pos = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
            if cubes.contains(&pos) {
                sides -= 1;
            }
        }
    }

    sides
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
