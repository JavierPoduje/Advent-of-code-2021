use std::str::FromStr;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    (part1(), part2())
}

struct Coord {
    x: u64,
    y: u64,
    aim: u64,
}

impl Coord {
    pub fn new() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }

    fn mult(&self) -> u64 {
        self.x * self.y
    }
}

enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl FromStr for Command {
    type Err = String;

    fn from_str(raw_command: &str) -> Result<Self, String> {
        if let Some((direction, units)) = raw_command.split_once(" ") {
            let units = units.parse().unwrap();

            Ok(match direction {
                "forward" => Command::Forward(units),
                "down" => Command::Down(units),
                "up" => Command::Up(units),
                _ => panic!("Unhandled direction"),
            })
        } else {
            Err("Couldn't format command".to_string())
        }
    }
}

pub fn part1() -> u64 {
    let commands = read_one_per_line::<Command>("./src/day_2/part1.txt").unwrap();
    commands
        .iter()
        .fold(Coord::new(), |coord, command| match command {
            Command::Forward(units) => Coord {
                x: coord.x + units,
                ..coord
            },
            Command::Down(units) => Coord {
                y: coord.y + units,
                ..coord
            },
            Command::Up(units) => Coord {
                y: coord.y - units,
                ..coord
            },
        })
        .mult()
}

pub fn part2() -> u64 {
    let commands = read_one_per_line::<Command>("./src/day_2/part2.txt").unwrap();
    commands
        .iter()
        .fold(Coord::new(), |coord, command| match command {
            Command::Forward(units) => Coord {
                x: coord.x + units,
                y: coord.y + coord.aim * units,
                ..coord
            },
            Command::Down(units) => Coord {
                aim: coord.aim + units,
                ..coord
            },
            Command::Up(units) => Coord {
                aim: coord.aim - units,
                ..coord
            },
        })
        .mult()
}
