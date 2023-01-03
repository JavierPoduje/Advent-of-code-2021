use std::cmp::max;
use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

type Matrix = Vec<Vec<Cell>>;

pub fn solution() -> (String, String) {
    let mut matrix: Matrix = read_one_per_line::<String>("./src/day_8/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.split("")
                .filter(|char| !char.is_empty())
                .map(|char| Cell::new(char))
                .collect()
        })
        .collect();

    let heights_by_coord = calculate_heights_by_coord(&matrix);
    set_visibility(&mut matrix, heights_by_coord);

    let part1 = calculate_part1(&matrix);

    (part1.to_string(), "B".to_string())
}

fn calculate_part1(matrix: &Matrix) -> u64 {
    let mut sum = 0;

    for (_row_idx, row) in matrix.iter().enumerate() {
        for (_col_idx, cell) in row.iter().enumerate() {
            if cell.visible {
                sum += 1;
            }
        }
    }

    sum
}

fn set_visibility(
    matrix: &mut Matrix,
    heights_by_coord: HashMap<(usize, usize), HeightByDirection>,
) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    for (row_idx, row) in matrix.iter_mut().enumerate() {
        for (col_idx, cell) in row.iter_mut().enumerate() {
            let heights = heights_by_coord.get(&(row_idx, col_idx)).unwrap();
            let is_edge =
                row_idx == 0 || row_idx == rows - 1 || col_idx == 0 || col_idx == cols - 1;
            cell.set_visibility(heights, is_edge);
        }
    }
}

fn calculate_heights_by_coord(matrix: &Matrix) -> HashMap<(usize, usize), HeightByDirection> {
    let mut height_by_direction = HashMap::new();

    let rows = matrix.len();
    let cols = matrix[0].len();

    for row in 0..rows {
        for col in 0..cols {
            height_by_direction.insert((row, col), HeightByDirection::new());
        }
    }

    // from top to bottom
    for col in 0..cols {
        let mut tallest_tree = 0;
        for row in 0..rows {
            height_by_direction.get_mut(&(row, col)).unwrap().up = tallest_tree;
            let current_value = matrix[row][col].value;
            tallest_tree = max(tallest_tree, current_value);
        }
    }

    // from right to left
    for row in 0..rows {
        let mut tallest_tree = 0;
        for col in (0..cols).rev() {
            height_by_direction.get_mut(&(row, col)).unwrap().right = tallest_tree;
            let current_value = matrix[row][col].value;
            tallest_tree = max(tallest_tree, current_value);
        }
    }

    // from bottom to top
    for col in 0..cols {
        let mut tallest_tree = 0;
        for row in (0..rows).rev() {
            height_by_direction.get_mut(&(row, col)).unwrap().down = tallest_tree;
            let current_value = matrix[row][col].value;
            tallest_tree = max(tallest_tree, current_value);
        }
    }

    // from left to right
    for row in 0..rows {
        let mut tallest_tree = 0;
        for col in 0..cols {
            height_by_direction.get_mut(&(row, col)).unwrap().left = tallest_tree;
            let current_value = matrix[row][col].value;
            tallest_tree = max(tallest_tree, current_value);
        }
    }

    height_by_direction
}

#[derive(Debug)]
struct HeightByDirection {
    up: i32,
    right: i32,
    down: i32,
    left: i32,
}

impl HeightByDirection {
    pub fn new() -> Self {
        Self {
            up: 0,
            right: 0,
            down: 0,
            left: 0,
        }
    }
}

#[derive(Debug)]
struct Cell {
    value: i32,
    visible: bool,
}

impl Cell {
    pub fn new(char: &str) -> Self {
        Self {
            value: char.parse::<i32>().unwrap(),
            visible: false,
        }
    }

    pub fn set_visibility(&mut self, height_by_direction: &HeightByDirection, is_edge: bool) {
        self.visible = is_edge
            || self.value > height_by_direction.up
            || self.value > height_by_direction.left
            || self.value > height_by_direction.right
            || self.value > height_by_direction.down;
    }
}
