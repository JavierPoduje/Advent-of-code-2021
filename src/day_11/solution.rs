use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

type Grid = Vec<Vec<u32>>;
type Coord = (usize, usize); // row, col

pub fn solution() -> (u64, u64) {
    let input = read_one_per_line::<String>("./src/day_11/input.txt").unwrap();
    let mut grid: Grid = build_grid(input);
    let mut flashes: u64 = 0;
    let mut first_complete_flash: i64 = -1;

    for step in 0.. {
        increase_all(&mut grid);
        let mut flashables: HashSet<Coord> = HashSet::new();

        while any_new_flashable(&grid, &flashables) {
            // identify new flashables
            let mut new_flashables: HashSet<Coord> = HashSet::new();
            for (row_idx, row) in grid.iter().enumerate() {
                for (col_idx, value) in row.iter().enumerate() {
                    if value > &9 && !flashables.contains(&(row_idx, col_idx)) {
                        flashables.insert((row_idx, col_idx));
                        new_flashables.insert((row_idx, col_idx));
                    }
                }
            }

            // update new_flashables values
            for coord in new_flashables {
                let neighbors = neighbors(&grid, coord);
                for (row, col) in neighbors {
                    grid[row][col] += 1;
                }
            }

            if flashables.len() == (grid.len() * grid[0].len()) {
                first_complete_flash = (step + 1) as i64;
            }

            // flash flashable cells
            for (row, col) in &flashables {
                grid[row.clone()][col.clone()] = 0;
            }
        }

        if step < 100 {
            flashes += flashables.len() as u64;
        }

        if first_complete_flash != -1 {
            break;
        }
    }

    (flashes, first_complete_flash as u64)
}

fn neighbors(grid: &Grid, (flashable_row, flashable_col): Coord) -> Vec<Coord> {
    let directions: Vec<(i8, i8)> = Vec::from([
        (-1, 0),  // up
        (-1, 1),  // up-right
        (0, 1),   // right
        (1, 1),   // bottom-right
        (1, 0),   // bottom
        (1, -1),  // bottom-left
        (0, -1),  // left
        (-1, -1), // up-left
    ]);

    let in_boundaries = |row, col| {
        let rows = grid.len() as isize;
        let cols = grid[0].len() as isize;
        0 <= rows && row < rows && 0 <= cols && col < cols
    };

    let mut neighbors = Vec::new();

    for dir in directions {
        let row = flashable_row as i8 + dir.0;
        let col = flashable_col as i8 + dir.1;
        if in_boundaries(isize::from(row as u8), isize::from(col as u8)) {
            neighbors.push((usize::from(row as u8), usize::from(col as u8)));
        }
    }

    neighbors
}

fn any_new_flashable(grid: &Grid, flashable_cells: &HashSet<Coord>) -> bool {
    for (row_idx, row) in grid.iter().enumerate() {
        if row
            .iter()
            .enumerate()
            .any(|(col_idx, value)| value > &9 && !flashable_cells.contains(&(row_idx, col_idx)))
        {
            return true;
        }
    }
    false
}

fn increase_all(grid: &mut Grid) {
    for row in grid.iter_mut() {
        for value in row.iter_mut() {
            *value = *value + 1;
        }
    }
}

fn build_grid(input: Vec<String>) -> Grid {
    input
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect()
}
