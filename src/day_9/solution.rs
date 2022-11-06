use std::collections::{HashSet, VecDeque};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let input = read_one_per_line::<String>("./src/day_9/input.txt").unwrap();
    let raw_grid: Vec<Vec<u32>> = input
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars()
                .map(|value| value.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let grid = build_grid(raw_grid);
    let lows: Vec<&Location> = Grid::get_lows(&grid);
    let part1 = sum_lows_up(&grid, &lows);
    let part2 = sum_basins_up(&grid, &lows);

    (part1 as u64, part2 as u64)
}

fn sum_basins_up(grid: &Vec<Vec<Location>>, lows: &Vec<&Location>) -> u32 {
    let mut basins_sums: Vec<u32> = Vec::new();
    for low in lows {
        basins_sums.push(sum_basin(grid, low));
    }
    basins_sums.sort();
    basins_sums
        .iter()
        .rev()
        .take(3)
        .fold(1, |acc, value| acc * value)
}

fn sum_basin(grid: &Vec<Vec<Location>>, loc: &Location) -> u32 {
    let mut hash = HashSet::new();
    hash.insert(&loc.id);

    let mut stack = VecDeque::new();
    stack.push_back(loc.clone());

    let mut basins = 1;

    while !stack.is_empty() {
        let curr = stack.pop_front().unwrap();
        let neighbors = get_neighbors(grid, curr.clone());

        for neighbor in neighbors {
            if !hash.contains(&neighbor.id) && neighbor.value != 9 && neighbor.value > curr.value {
                hash.insert(&neighbor.id);
                basins += 1;
                stack.push_back(neighbor.clone());
            }
        }
    }

    basins
}

fn get_neighbors<'a>(grid: &'a Vec<Vec<Location>>, curr: Location) -> Vec<&'a Location> {
    let mut neighbors = Vec::new();

    // up
    if curr.row > 0 {
        neighbors.push(&grid[curr.row - 1][curr.col]);
    }
    // right
    if curr.col < grid[0].len() - 1 {
        neighbors.push(&grid[curr.row][curr.col + 1]);
    }
    // down
    if curr.row < grid.len() - 1 {
        neighbors.push(&grid[curr.row + 1][curr.col]);
    }
    // left
    if curr.col > 0 {
        neighbors.push(&grid[curr.row][curr.col - 1]);
    }

    neighbors
}

fn sum_lows_up(_grid: &Vec<Vec<Location>>, lows: &Vec<&Location>) -> u32 {
    lows.into_iter()
        .fold(0, |acc, location| acc + location.value + 1)
}

struct Grid {}

impl Grid {
    pub fn get_lows(grid: &Vec<Vec<Location>>) -> Vec<&Location> {
        let mut lows: Vec<&Location> = vec![];
        for row in grid {
            for location in row {
                if Grid::locaton_is_low(grid, &location) {
                    lows.push((&location).clone());
                }
            }
        }
        lows
    }

    pub fn locaton_is_low(grid: &Vec<Vec<Location>>, location: &Location) -> bool {
        let directions: Vec<Vec<i32>> = vec![
            vec![-1, 0], // up
            vec![0, 1],  // right
            vec![1, 0],  // down
            vec![0, -1], // left
        ];

        for dir in directions {
            let row = location.row as i32 + dir[0];
            let col = location.col as i32 + dir[1];
            if Self::in_boundaries(&grid, row, col) {
                let other = &grid[row as usize][col as usize];
                if other.value <= location.value {
                    return false;
                }
            }
        }

        true
    }

    fn in_boundaries(grid: &Vec<Vec<Location>>, row: i32, col: i32) -> bool {
        0 <= row && row < grid.len() as i32 && 0 <= col && col < grid[0].len() as i32
    }
}

#[derive(Debug, Clone)]
struct Location {
    id: String,
    value: u32,
    row: usize,
    col: usize,
}

impl Location {
    pub fn new(value: u32, row: usize, col: usize) -> Self {
        let id = format!("{}-{}", row.to_string(), col.to_string());
        Self {
            id,
            value,
            row,
            col,
        }
    }
}

fn build_grid(raw_grid: Vec<Vec<u32>>) -> Vec<Vec<Location>> {
    let mut grid: Vec<Vec<Location>> = Vec::new();
    for (row_idx, row) in raw_grid.iter().enumerate() {
        let mut grid_row = Vec::new();
        for (col_idx, value) in row.iter().enumerate() {
            grid_row.push(Location::new(value.clone(), row_idx, col_idx));
        }
        grid.push(grid_row);
    }
    grid
}
