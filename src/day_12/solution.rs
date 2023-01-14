use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let (grid, start, end, width, height) = parse();

    let part1 = part1(grid.clone(), start, end, width, height);
    let part2 = part2(grid, start, end, width, height);

    (part1.to_string(), part2.to_string())
}

fn part1(
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
) -> usize {
    find_shortest(grid, start, end, width, height).unwrap()
}

fn part2(
    grid: Vec<Vec<u8>>,
    _: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
) -> usize {
    let mut startpoints = Vec::new();

    for (row, line) in grid.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == b'a' {
                startpoints.push((row, col));
            }
        }
    }

    startpoints
        .iter()
        .filter_map(|p| find_shortest(grid.clone(), p.clone(), end, width, height))
        .min()
        .unwrap()
}

fn find_shortest(
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
) -> Option<usize> {
    let mut to_visit = Vec::new();

    to_visit.extend(get_sorrounding_points(start, width, height));

    let mut shortest: HashMap<(usize, usize), usize> = HashMap::new();

    shortest.insert(start, 0);

    while let Some(loc) = to_visit.pop() {
        // find locations around us that can get to 'loc'
        let cur_elevation = grid[loc.0][loc.1];
        let points = get_sorrounding_points(loc, width, height);
        let valid = points
            .iter()
            .filter(|pos| grid[pos.0][pos.1] + 1 >= cur_elevation)
            .copied()
            .collect::<Vec<(usize, usize)>>();

        let new_path_dist = valid.iter().filter_map(|pos| shortest.get(pos)).min();

        if new_path_dist.is_none() {
            continue;
        }

        let new_path_dist = new_path_dist.unwrap() + 1;

        let cur_path_dist = shortest.entry(loc).or_insert(usize::MAX);
        if *cur_path_dist > new_path_dist {
            *cur_path_dist = new_path_dist;
            to_visit.extend(points.iter());
        }
    }

    shortest.get(&end).copied()
}

fn get_sorrounding_points(pos: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let ipos = (pos.0 as i32, pos.1 as i32);
    dir.iter()
        .map(|d| (ipos.0 + d.0, ipos.1 + d.1))
        .filter(|pos| pos.0 >= 0 && pos.1 >= 0 && pos.0 < height as i32 && pos.1 < width as i32)
        .map(|pos| (pos.0 as usize, pos.1 as usize))
        .collect()
}

fn parse() -> (Vec<Vec<u8>>, (usize, usize), (usize, usize), usize, usize) {
    let mut grid = Vec::new();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_12/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    for (row, line) in lines.iter().enumerate() {
        let mut gridline = line.chars().map(|c| c as u8).collect::<Vec<u8>>();

        if let Some(start_point) = gridline.iter().position(|p| p == &b'S') {
            start = (row, start_point);
            gridline[start_point] = b'a';
        }

        if let Some(end_point) = gridline.iter().position(|p| p == &b'E') {
            end = (row, end_point);
            gridline[end_point] = b'z';
        }

        grid.push(gridline);
    }

    let width = grid[0].len();
    let height = grid.len();

    (grid, start, end, width, height)
}
