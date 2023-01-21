use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let (map, steps) = parse();
    let part1 = part1(map.clone(), steps.clone());
    let part2 = part2(map, steps);
    (part1.to_string(), part2.to_string())
}

fn part1(map: Map, steps: Vec<Move>) -> i64 {
    let mut start_col = 0;
    for col in 0..map.width {
        if Some(&Tile::Space) == map.tiles.get(&(0, col)) {
            start_col = col as i64;
        }
    }

    let mut pos = (0, start_col);
    let mut facing = 0;

    for step in steps {
        match step {
            Move::Right => facing = (facing + 1) % Map::DIR.len(),
            Move::Left => facing = (facing + Map::DIR.len() - 1) % Map::DIR.len(),
            Move::Forward(n) => {
                for _ in 0..n {
                    let dir = Map::DIR[facing];
                    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

                    if let Some(tile) = map.tiles.get(&next_pos) {
                        match tile {
                            Tile::Space => pos = next_pos,
                            Tile::Wall => break,
                        }
                    } else {
                        pos = map.wrap(&pos, facing);
                    }
                }
            }
        }
    }

    (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + facing as i64
}

fn part2(map: Map, steps: Vec<Move>) -> i64 {
    let mut start_col = 0;

    for col in 0..map.width {
        if Some(&Tile::Space) == map.tiles.get(&(0, col)) {
            start_col = col as i64;
            break;
        }
    }

    let mut pos = (0, start_col);
    let mut facing = 0;

    for step in steps {
        match step {
            Move::Right => facing = (facing + 1) % Map::DIR.len(),
            Move::Left => facing = (facing + Map::DIR.len() - 1) % Map::DIR.len(),
            Move::Forward(n) => {
                for _ in 0..n {
                    let dir = Map::DIR[facing];
                    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

                    if let Some(tile) = map.tiles.get(&next_pos) {
                        match tile {
                            Tile::Space => pos = next_pos,
                            Tile::Wall => break,
                        }
                    } else {
                        let (new_facing, new_pos) = map.wrap_cube(&pos, facing);
                        facing = new_facing;
                        pos = new_pos;
                    }
                }
            }
        }
    }

    (pos.0 + 1) * 1000 + (pos.1 + 1) * 4 + facing as i64
}

fn parse() -> (Map, Vec<Move>) {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_22/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let mut map = Map::default();
    let mut steps = Vec::new();

    for (row, line) in lines.iter().enumerate() {
        let ch = line.chars().collect::<Vec<_>>();
        if ch[0].is_ascii_digit() {
            let mut n = 0;
            for c in ch {
                match c {
                    '0'..='9' => n = (n * 10) + (c as u8 - b'0') as i64,
                    m => {
                        steps.push(Move::Forward(n));
                        n = 0;
                        match m {
                            'R' => steps.push(Move::Right),
                            'L' => steps.push(Move::Left),
                            _ => panic!("char not matched '{m}'"),
                        }
                    }
                }
            }
            steps.push(Move::Forward(n));
        } else {
            for (col, c) in ch.iter().enumerate() {
                match c {
                    ' ' => {}
                    '#' => map.insert((row as i64, col as i64), Tile::Wall),
                    '.' => map.insert((row as i64, col as i64), Tile::Space),
                    _ => panic!("unkown char '{c}'"),
                }
            }
        }
    }

    (map, steps)
}

#[derive(Debug, Default, Clone)]
struct Map {
    tiles: HashMap<(i64, i64), Tile>,
    width: i64,
    height: i64,
}

impl Map {
    const DIR: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn insert(&mut self, loc: (i64, i64), tile: Tile) {
        self.tiles.insert(loc, tile);
        self.height = self.height.max(loc.0);
        self.width = self.width.max(loc.1);
    }

    fn wrap(&self, pos: &(i64, i64), facing: usize) -> (i64, i64) {
        let mut result = *pos;
        match facing {
            0 => result.1 = 0,
            1 => result.0 = 0,
            2 => result.1 = self.width,
            3 => result.0 = self.height,
            _ => panic!("bad facing"),
        }

        let dir = Self::DIR[facing];

        while !self.tiles.contains_key(&result) {
            result = (result.0 + dir.0, result.1 + dir.1);
        }

        match self.tiles.get(&result).unwrap() {
            Tile::Wall => *pos,
            Tile::Space => result,
        }
    }

    fn wrap_cube(&self, pos: &(i64, i64), facing: usize) -> (usize, (i64, i64)) {
        let current_face = self.face(pos);
        let row = pos.0;
        let col = pos.1;

        let (new_face, new_facing, new_pos) = match (current_face, facing) {
            (1, 0) => (3, 3, (49, row + 50)),
            (1, 2) => (4, 1, (100, row - 50)),
            (2, 3) => (6, 0, (col + 100, 0)),
            (2, 2) => (4, 0, (149 - row, 0)),
            (3, 3) => (6, 3, (199, col - 100)),
            (3, 0) => (5, 2, (149 - row, 99)),
            (3, 1) => (1, 2, (col - 50, 99)),
            (4, 3) => (1, 0, (col + 50, 50)),
            (4, 2) => (2, 0, (149 - row, 50)),
            (5, 0) => (3, 2, (149 - row, 149)),
            (5, 1) => (6, 2, (col + 100, 49)),
            (6, 0) => (5, 3, (149, row - 100)),
            (6, 1) => (3, 1, (0, col + 100)),
            (6, 2) => (2, 1, (0, row - 100)),
            _ => panic!("weird location {current_face}, {facing}"),
        };

        assert!(self.face(&new_pos) == new_face);

        if matches!(self.tiles.get(&new_pos).unwrap(), Tile::Wall) {
            (facing, *pos)
        } else {
            (new_facing, new_pos)
        }
    }

    fn face(&self, pos: &(i64, i64)) -> usize {
        let face_index = (pos.0 / 50) * 3 + pos.1 / 50;
        [0, 2, 3, 0, 1, 0, 4, 5, 0, 6][face_index as usize]
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Space,
    Wall,
}

#[derive(Debug, Clone)]
enum Move {
    Forward(i64),
    Left,
    Right,
}
