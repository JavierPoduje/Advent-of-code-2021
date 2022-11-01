use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() {
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
    let part1 = lows
        .into_iter()
        .fold(0, |acc, location| acc + location.value + 1);

    println!("part1: {}", part1);
}

struct Grid {}

impl Grid {
    pub fn get_lows(grid: &Vec<Vec<Location>>) -> Vec<&Location> {
        let mut lows: Vec<&Location> = vec![];
        for row in grid {
            for location in row {
                if Grid::locaton_is_low(grid, &location) {
                    lows.push(location.clone());
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

#[derive(Debug)]
struct Location {
    value: u32,
    row: usize,
    col: usize,
    is_low: bool,
}

impl Location {
    pub fn new(value: u32, row: usize, col: usize) -> Self {
        Self {
            value,
            row,
            col,
            is_low: false,
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
