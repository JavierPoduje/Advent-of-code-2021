use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let valley = parse();
    let (first_trip, part1) = part1(valley);
    let part2 = part2(first_trip, part1);
    (part1.to_string(), part2.to_string())
}

fn part1(valley: Blizzard) -> (Blizzard, usize) {
    let (result, time) = search(&valley).unwrap();
    (result, time)
}

fn part2(mut return_trip: Blizzard, first_trip_time: usize) -> usize {
    return_trip.find_start = true;
    let (mut back_to_end, return_time) = search(&return_trip).unwrap();
    println!("return time = {return_time}");

    back_to_end.find_start = false;
    let final_time = search(&back_to_end).unwrap().1;
    println!("final time = {final_time}");

    first_trip_time + return_time + final_time
}

fn parse() -> Blizzard {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_24/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let width: i32 = lines[0].len() as i32;
    let height: i32 = lines.len() as i32;
    let mut valley = Blizzard::new(width, height);

    let mut idx = 0;
    for line in lines {
        for c in line.chars() {
            match c {
                '#' => valley.map[idx] = Blizzard::WALL,
                '^' => valley.map[idx] = Blizzard::UP,
                'v' => valley.map[idx] = Blizzard::DOWN,
                '<' => valley.map[idx] = Blizzard::LEFT,
                '>' => valley.map[idx] = Blizzard::RIGHT,
                '.' => {}
                _ => panic!("unknown char '{c}'"),
            }
            idx += 1;
        }
    }

    valley
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    width: i32,
    height: i32,
    map: Vec<u8>,
    player_pos: (i32, i32),
    find_start: bool,
}

impl Blizzard {
    const UP: u8 = 0x01;
    const DOWN: u8 = 0x02;
    const LEFT: u8 = 0x04;
    const RIGHT: u8 = 0x08;
    const WALL: u8 = 0x80;

    const LOOK: [((i32, i32), u8); 4] = [
        ((-1, 0), Self::DOWN),
        ((1, 0), Self::UP),
        ((0, -1), Self::RIGHT),
        ((0, 1), Self::LEFT),
    ];

    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            map: vec![0; (width * height) as usize],
            player_pos: (0, 1),
            find_start: false,
        }
    }

    fn get(&self, row: i32, col: i32) -> u8 {
        self.map[(row * self.width + col) as usize]
    }

    fn wrap(&self, from: (i32, i32), look: (i32, i32)) -> (i32, i32) {
        match look {
            (-1, 0) => (self.height - 2, from.1),
            (1, 0) => (1, from.1),
            (0, -1) => (from.0, self.width - 2),
            (0, 1) => (from.0, 1),
            _ => panic!("bad location {look:?}"),
        }
    }

    fn tick(&self) -> Self {
        let mut map: Vec<u8> = Vec::new();

        let mut idx = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                map.push(0);
                let tile = self.get(row, col);
                if row == 0 || row > self.height - 2 || col == 0 || col > self.width - 2 {
                    map[idx] = tile;
                } else {
                    assert!(row > 0 && row < self.height - 1, "{row},{col}");
                    assert!(col > 0 && col < self.width - 1, "{row},{col}");
                    for l in Self::LOOK {
                        let lrow = row + l.0 .0;
                        let lcol = col + l.0 .1;
                        let tile = self.get(lrow, lcol);
                        if tile == Self::WALL {
                            let (lrow, lcol) = self.wrap((row, col), l.0);
                            map[idx] |= self.get(lrow, lcol) & l.1;
                        } else {
                            map[idx] |= tile & l.1;
                        }
                    }
                }
                idx += 1;
            }
        }

        Self {
            width: self.width,
            height: self.height,
            map,
            player_pos: self.player_pos,
            find_start: self.find_start,
        }
    }

    fn _draw(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!(
                    "{}",
                    match self.get(row, col) {
                        0 => '.',
                        Self::UP => '^',
                        Self::DOWN => 'v',
                        Self::LEFT => '<',
                        Self::RIGHT => '>',
                        Self::WALL => '#',
                        _ => 'X',
                    }
                );
            }
            println!();
        }
    }
}

impl Searcher for Blizzard {
    fn moves(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut moves = Vec::new();

        let new_state = self.tick();
        let stay_put = (self.player_pos.0, self.player_pos.1);
        let move_up = (self.player_pos.0 - 1, self.player_pos.1);
        let move_down = (self.player_pos.0 + 1, self.player_pos.1);
        let move_left = (self.player_pos.0, self.player_pos.1 - 1);
        let move_right = (self.player_pos.0, self.player_pos.1 + 1);

        for new_pos in [stay_put, move_up, move_down, move_left, move_right] {
            if new_pos.0 >= 0 && new_pos.0 < self.height {
                if new_state.get(new_pos.0, new_pos.1) == 0 {
                    let mut valid = new_state.clone();
                    valid.player_pos = new_pos;
                    moves.push(valid);
                }
            }
        }

        moves
    }

    fn is_win_state(&self) -> bool {
        if self.find_start {
            self.player_pos == (0, 1)
        } else {
            self.player_pos == (self.height - 1, self.width - 2)
        }
    }
}

trait Searcher {
    fn moves(&self) -> Vec<Self>
    where
        Self: Sized;
    fn is_win_state(&self) -> bool;
}

fn search<T: Searcher + Clone + Eq + Hash>(start: &T) -> Option<(T, usize)> {
    let mut q: HashSet<T> = HashSet::new();

    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut index: HashSet<T> = HashSet::new();
    let mut target = None;

    index.insert(start.clone());
    q.insert(start.clone());

    dist.insert(start.clone(), 0);

    while !q.is_empty() {
        let u = q
            .iter()
            .map(|item| (item, dist.get(item).unwrap()))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0
            .clone();

        if u.is_win_state() {
            target = Some(u);
            break;
        }

        if !q.remove(&u) {
            panic!("treid to remove u from q but failed");
        }

        for m in u.moves() {
            let v = if q.contains(&m) {
                m
            } else if index.insert(m.clone()) {
                dist.insert(m.clone(), usize::MAX);
                q.insert(m.clone());
                m
            } else {
                continue;
            };

            let alt = dist.get(&u).unwrap() + 1;
            let dist_v = *dist.get(&v).unwrap();
            if alt < dist_v {
                dist.insert(v.clone(), alt);
            }
        }
    }

    let target = target?;

    Some((target.clone(), *dist.get(&target)?))
}
