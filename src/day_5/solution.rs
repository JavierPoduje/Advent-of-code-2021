use super::super::utils::read_one_per_line::read_one_per_line;
use std::cmp::max;

#[derive(Debug, Clone)]
struct Coord {
    x: u64,
    y: u64,
}

impl Coord {
    pub fn new(raw: &str) -> Self {
        let pair: Vec<u64> = raw
            .split(',')
            .map(|value| value.parse::<u64>().unwrap())
            .collect();
        Self {
            x: pair[0],
            y: pair[1],
        }
    }

    pub fn move_to(&mut self, other: &Coord) {
        // move x
        if self.x > other.x {
            self.x -= 1;
        } else if self.x < other.x {
            self.x += 1;
        }

        // move y
        if self.y > other.y {
            self.y -= 1;
        } else if self.y < other.y {
            self.y += 1;
        }
    }
}

#[derive(Debug, Clone)]
struct Movement {
    start: Coord,
    end: Coord,
}

impl Movement {
    pub fn new(raw_start: &str, raw_end: &str) -> Self {
        Self {
            start: Coord::new(raw_start),
            end: Coord::new(raw_end),
        }
    }
}

pub fn solution() -> (u64, u64) {
    let input = read_one_per_line::<String>("./src/day_5/input.txt").unwrap();
    let raw_movements = build_movements(input);
    let filtered_movements: Vec<Movement> = raw_movements
        .clone()
        .into_iter()
        .filter(|mov| mov.start.x == mov.end.x || mov.start.y == mov.end.y)
        .collect();
    let all_movements: Vec<Movement> = raw_movements.into_iter().collect();

    let mut canvas_one = build_canvas(&filtered_movements);
    let mut canvas_two = canvas_one.clone();

    for movement in &filtered_movements {
        walk(&movement, &mut canvas_one);
    }
    for movement in &all_movements {
        walk(&movement, &mut canvas_two);
    }

    let part1 = count_overlaps(&canvas_one);
    let part2 = count_overlaps(&canvas_two);

    (part1, part2)
}

fn walk(movement: &Movement, canvas: &mut Vec<Vec<u64>>) {
    let mut start = movement.start.clone();
    let end = movement.end.clone();

    while start.x != end.x || start.y != end.y {
        canvas[start.x as usize][start.y as usize] += 1;
        start.move_to(&end);
    }

    canvas[end.x as usize][end.y as usize] += 1;
}

fn count_overlaps(canvas: &Vec<Vec<u64>>) -> u64 {
    let mut overlaps = 0;
    for row in canvas {
        for value in row {
            if value > &1 {
                overlaps += 1;
            }
        }
    }
    overlaps
}

fn build_movements(input: Vec<String>) -> Vec<Movement> {
    input
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|raw_movement| {
            let pair: Vec<&str> = raw_movement.split(" -> ").collect();
            Movement::new(pair[0], pair[1])
        })
        .collect()
}

fn build_canvas(movements: &Vec<Movement>) -> Vec<Vec<u64>> {
    let mut max_x: u64 = 0;
    let mut max_y: u64 = 0;

    for mov in movements {
        max_x = max(max_x, max(mov.start.x, mov.end.x));
        max_y = max(max_y, max(mov.start.y, mov.end.y));
    }

    let mut canvas: Vec<Vec<u64>> = vec![];
    for _ in 0..=max_x {
        let mut row: Vec<u64> = vec![];
        for _ in 0..=max_y {
            row.push(0);
        }
        canvas.push(row);
    }

    canvas
}

#[test]
fn horizontal() {
    let mut canvas: Vec<Vec<u64>> = [
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
    ]
    .to_vec();
    walk(
        &Movement {
            start: Coord { x: 0, y: 0 },
            end: Coord { x: 0, y: 3 },
        },
        &mut canvas,
    );
    assert_eq!(
        canvas,
        [[1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0],]
    );
}

#[test]
fn vertical() {
    let mut canvas: Vec<Vec<u64>> = [
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
    ]
    .to_vec();
    walk(
        &Movement {
            start: Coord { x: 2, y: 2 },
            end: Coord { x: 0, y: 2 },
        },
        &mut canvas,
    );
    assert_eq!(
        canvas,
        [
            [0, 0, 1, 0],
            [0, 0, 1, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 0],
        ]
    );
}

#[test]
fn vertical_2() {
    let mut canvas: Vec<Vec<u64>> = [
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
    ]
    .to_vec();
    walk(
        &Movement {
            start: Coord { x: 1, y: 3 },
            end: Coord { x: 3, y: 3 },
        },
        &mut canvas,
    );
    assert_eq!(
        canvas,
        [
            [0, 0, 0, 0],
            [0, 0, 0, 1],
            [0, 0, 0, 1],
            [0, 0, 0, 1],
        ]
    );
}


#[test]
fn diagonal() {
    let mut canvas: Vec<Vec<u64>> = [
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
    ]
    .to_vec();
    walk(
        &Movement {
            start: Coord { x: 1, y: 1 },
            end: Coord { x: 3, y: 3 },
        },
        &mut canvas,
    );
    assert_eq!(
        canvas,
        [[0, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1],]
    );
}

#[test]
fn diagonal_2() {
    let mut canvas: Vec<Vec<u64>> = [
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
        [0 as u64, 0 as u64, 0 as u64, 0 as u64].to_vec(),
    ]
    .to_vec();
    walk(
        &Movement {
            start: Coord { x: 3, y: 0 },
            end: Coord { x: 1, y: 2 },
        },
        &mut canvas,
    );
    assert_eq!(
        canvas,
        [
            [0, 0, 0, 0],
            [0, 0, 1, 0],
            [0, 1, 0, 0],
            [1, 0, 0, 0],
        ]
    );
}
