use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let rows: Vec<(Direction, usize)> = read_one_per_line::<String>("./src/day_9/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            let splitted: Vec<&str> = row.split(" ").collect();

            let char = splitted[0];
            let steps = splitted[1];

            (Direction::new(char), steps.parse::<usize>().unwrap())
        })
        .collect();

    let mut coords: Vec<Coord> = vec![Coord::new(); 10];

    for (dir, steps) in rows {
        for _ in 0..steps {
            for curr_coord_idx in 0..coords.len() - 1 {
                if curr_coord_idx == 0 {
                    match dir {
                        Direction::U => coords[curr_coord_idx].up(),
                        Direction::R => coords[curr_coord_idx].right(),
                        Direction::D => coords[curr_coord_idx].down(),
                        Direction::L => coords[curr_coord_idx].left(),
                    }
                }

                let head = coords[curr_coord_idx].clone();
                coords[curr_coord_idx + 1].move_to(&head);
            }
        }
    }

    (
        coords[1].visited.len().to_string(),
        coords[coords.len() - 1].visited.len().to_string(),
    )
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
    visited: HashSet<(i64, i64)>,
}

impl Coord {
    pub fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        Self {
            x: 0,
            y: 0,
            visited,
        }
    }

    pub fn move_to(&mut self, head: &Self) {
        if self.close_enough_to(head) {
            return;
        }

        if self.x < head.x && self.y == head.y {
            self.up();
        } else if self.x < head.x && self.y < head.y {
            self.up_right();
        } else if self.x == head.x && self.y < head.y {
            self.right();
        } else if self.x > head.x && self.y < head.y {
            self.down_right();
        } else if self.x > head.x && self.y == head.y {
            self.down();
        } else if self.x > head.x && self.y > head.y {
            self.left_down();
        } else if self.x == head.x && self.y > head.y {
            self.left();
        } else if self.x < head.x && self.y > head.y {
            self.left_up();
        }
    }

    fn up(&mut self) {
        self.x += 1;
        self.visit();
    }

    fn up_right(&mut self) {
        self.x += 1;
        self.y += 1;
        self.visit();
    }

    fn right(&mut self) {
        self.y += 1;
        self.visit();
    }

    fn down_right(&mut self) {
        self.x -= 1;
        self.y += 1;
        self.visit();
    }

    fn down(&mut self) {
        self.x -= 1;
        self.visit();
    }

    fn left_down(&mut self) {
        self.x -= 1;
        self.y -= 1;
        self.visit();
    }

    fn left(&mut self) {
        self.y -= 1;
        self.visit();
    }

    fn left_up(&mut self) {
        self.x += 1;
        self.y -= 1;
        self.visit();
    }

    fn visit(&mut self) {
        self.visited.insert((self.x, self.y));
    }

    fn close_enough_to(&self, other: &Self) -> bool {
        Self::abs_diff(self.x, other.x) <= 1 && Self::abs_diff(self.y, other.y) <= 1
    }

    fn abs_diff(a: i64, b: i64) -> i64 {
        std::cmp::max(a, b) - std::cmp::min(a, b)
    }
}

#[derive(Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    pub fn new(char: &str) -> Self {
        match char {
            "U" => Self::U,
            "R" => Self::R,
            "D" => Self::D,
            "L" => Self::L,
            _ => unreachable!(),
        }
    }
}
