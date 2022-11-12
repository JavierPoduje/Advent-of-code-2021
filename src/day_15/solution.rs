use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

type Matrix = Vec<Vec<u64>>;
type Coord = (usize, usize);
type Distances = HashMap<Coord, Option<u64>>;
type Seen = HashMap<Coord, bool>;

pub fn solution() -> (u64, u64) {
    let matrix: Matrix = read_one_per_line::<String>("./src/day_15/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            row.chars()
                .map(|value| value.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let distance = walk(matrix);

    (distance, 0)
}

fn walk(matrix: Matrix) -> u64 {
    let mut seen = build_seen(&matrix);
    let mut distances = build_distances(&matrix);

    while has_unvisited(&seen, &distances) {
        let curr = get_lowest_unvisited(&seen, distances.clone());
        seen.insert(curr.clone(), true);

        for neighbor in neighbors(&curr, &matrix) {
            if *seen.get(&neighbor).unwrap() {
                continue;
            }

            let weight = matrix[neighbor.0][neighbor.1];
            let curr_distance = distances.get(&curr).unwrap();

            let candidate_distance = curr_distance.unwrap() + weight;

            match distances.get(&neighbor) {
                Some(maybe_neighbor_distance) => match maybe_neighbor_distance {
                    Some(neighbor_distance) => {
                        if candidate_distance < *neighbor_distance {
                            distances.insert(neighbor, Some(candidate_distance));
                        }
                    }
                    None => {
                        distances.insert(neighbor, Some(candidate_distance));
                    }
                },
                None => {
                    distances.insert(neighbor, Some(candidate_distance));
                }
            }
        }
    }

    distances
        .get(&(matrix.len() - 1, matrix[0].len() - 1))
        .unwrap()
        .unwrap()
}

fn neighbors(coord: &Coord, matrix: &Matrix) -> Vec<Coord> {
    let mut neighbors: Vec<Coord> = Vec::new();
    let directions: Vec<(isize, isize)> = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    for dir in directions {
        let n_coord = (coord.0 as isize + dir.0, coord.1 as isize + dir.1);
        if !in_boundaries(&n_coord, matrix) {
            continue;
        }
        neighbors.push((n_coord.0 as usize, n_coord.1 as usize));
    }

    neighbors
}

fn in_boundaries(coord: &(isize, isize), matrix: &Matrix) -> bool {
    0 <= coord.0
        && coord.0 < matrix.len() as isize
        && 0 <= coord.1
        && coord.1 < matrix[0].len() as isize
}

fn get_lowest_unvisited(seen: &Seen, distances: Distances) -> Coord {
    distances
        .into_iter()
        .fold(((0, 0), None), |acc, (coord, dist)| {
            match seen.get(&coord) {
                Some(has_been_seen) => {
                    if *has_been_seen {
                        return acc;
                    }
                }
                None => {}
            }

            if dist.is_none() {
                return acc;
            }

            match acc.1 {
                Some(acc_distance) => {
                    if dist.unwrap() < acc_distance {
                        (coord, dist.clone())
                    } else {
                        acc
                    }
                }
                None => (coord, dist.clone()),
            }
        })
        .0
}

fn has_unvisited(seen: &Seen, distances: &Distances) -> bool {
    seen.into_iter()
        .any(|(coord, has_been_seen)| !has_been_seen && distances.get(&coord).is_some())
}

fn build_distances(matrix: &Matrix) -> Distances {
    let mut distances: Distances = HashMap::new();

    for (row_idx, row) in matrix.into_iter().enumerate() {
        for (col_idx, _) in row.into_iter().enumerate() {
            if row_idx == 0 && col_idx == 0 {
                distances.insert((row_idx, col_idx), Some(0));
            } else {
                distances.insert((row_idx, col_idx), None);
            }
        }
    }

    distances
}

fn build_seen(matrix: &Matrix) -> Seen {
    let mut seen: Seen = HashMap::new();

    for (row_idx, row) in matrix.into_iter().enumerate() {
        for (col_idx, _) in row.into_iter().enumerate() {
            seen.insert((row_idx, col_idx), false);
        }
    }

    seen
}
