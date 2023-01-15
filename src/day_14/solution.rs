use std::{cmp::max, collections::HashMap};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let (cave, bottom) = parse();
    let count = part1(cave, bottom);
    (count.to_string(), "B".to_string())
}

fn part1(mut cave: Cave, bottom: i32) -> i32 {
    let mut count = 0;
    while cave.drop_one(bottom) {
        count += 1;
    }
    cave.display();
    count
}

fn parse() -> (Cave, i32) {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_14/input.txt")
        .unwrap()
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect();

    let mut cave = Cave::default();
    let mut bottom = 0;

    for line in lines {
        let mut iter = line.split(" -> ");
        let mut start = Cave::convert(iter.next()).unwrap();
        while let Some(end) = Cave::convert(iter.next()) {
            cave.draw_line(start, end);
            bottom = max(start.1, end.1);
            start = end;
        }
    }

    (cave, bottom)
}

#[derive(Default, Clone)]
struct Cave {
    tile: HashMap<(i32, i32), char>,
}

impl Cave {
    pub fn convert(s: Option<&str>) -> Option<(i32, i32)> {
        if let Some(s) = s {
            let (x, y) = s.split_once(',').unwrap();
            Some((x.parse().unwrap(), y.parse().unwrap()))
        } else {
            None
        }
    }

    pub fn drop_one(&mut self, bottom: i32) -> bool {
        let mut sand = (500, 0);
        while let Some(next_pos) = self.fall(sand) {
            if next_pos.1 > bottom {
                return false;
            }
            sand = next_pos;
        }
        self.tile.insert(sand, 'o');
        true
    }

    fn fall(&self, pos: (i32, i32)) -> Option<(i32, i32)> {
        for delta_x in [0, -1, 1] {
            let new_pos = (pos.0 + delta_x, pos.1 + 1);
            if !self.tile.contains_key(&new_pos) {
                return Some(new_pos);
            }
        }

        None
    }

    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32)) {
        let dx = (end.0 - start.0).signum();
        let dy = (end.1 - start.1).signum();

        let mut point = start;
        self.tile.insert(point, '#');

        while point != end {
            point.0 += dx;
            point.1 += dy;
            self.tile.insert(point, '#');
        }

        println!("Draw line from {start:?} to {end:?}");
    }

    fn display(&self) {
        for y in 0..=9 {
            for x in 494..=503 {
                print!(
                    "{}",
                    if x == 500 && y == 0 {
                        '+'
                    } else if let Some(ch) = self.tile.get(&(x, y)) {
                        *ch
                    } else {
                        '.'
                    }
                );
            }
            println!("");
        }
        println!("");
    }
}
