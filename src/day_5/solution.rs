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
}

#[derive(Debug)]
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

pub fn solution() -> u64 {
    let input = read_one_per_line::<String>("./src/day_5/input.txt").unwrap();
    let movements: Vec<Movement> = build_movements(input)
        .into_iter()
        .filter(|mov| mov.start.x == mov.end.x || mov.start.y == mov.end.y)
        .collect();
    let mut canvas = build_canvas(&movements);

    for movement in &movements {
        walk(&movement, &mut canvas);
    }

    count_overlaps(&canvas)
}

fn walk(movement: &Movement, canvas: &mut Vec<Vec<u64>>) {
    let mut start = movement.start.clone();
    let mut end = movement.end.clone();

    if start.x == end.x {
        if start.y > end.y {
            while end.y <= start.y {
                canvas[end.x as usize][end.y as usize] += 1;
                end.y += 1;
            }
        } else {
            while start.y <= end.y {
                canvas[start.x as usize][start.y as usize] += 1;
                start.y += 1;
            }
        }
    } else {
        if start.x > end.x {
            while end.x <= start.x {
                canvas[end.x as usize][end.y as usize] += 1;
                end.x += 1;
            }
        } else {
            while start.x <= end.x {
                canvas[start.x as usize][start.y as usize] += 1;
                start.x += 1;
            }
        }
    }
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
