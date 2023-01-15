use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

const PART1_ROW: i64 = 2000000;

pub fn solution() -> (String, String) {
    let sensors = parse();
    let part1 = part1(sensors);
    (part1.to_string(), "B".to_string())
}

fn part1(sensors: Vec<Sensor>) -> usize {
    let mut coverage = HashSet::new();
    for s in sensors.clone() {
        let radius = s.radius();
        let dist = (s.loc.1 - PART1_ROW).abs();
        if dist > radius {
            continue;
        }

        let remainder = radius - dist;
        let left_x = s.loc.0 - remainder;
        let right_x = s.loc.0 + remainder;

        for x in left_x..=right_x {
            coverage.insert(x);
        }
    }
    let beacons: HashSet<i64> = HashSet::from_iter(
        sensors
            .iter()
            .filter(|s| s.beacon.1 == PART1_ROW)
            .map(|s| s.beacon.0)
            .collect::<Vec<i64>>(),
    );

    coverage.len() - beacons.len()
}

fn parse() -> Vec<Sensor> {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_15/input.txt")
        .unwrap()
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect();

    let mut sensors = Vec::new();

    for line in lines {
        let sensor = Sensor::parse(&line);
        sensors.push(sensor);
    }

    sensors
}

#[derive(Default, Debug, Clone)]
struct Sensor {
    loc: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn parse(s: &str) -> Self {
        let (left, beacon) = s.split_once(": closest beacon is at x=").unwrap();
        let (_, sensor) = left.split_once("Sensor at x=").unwrap();
        Self {
            loc: Self::coord(sensor),
            beacon: Self::coord(beacon),
        }
    }

    fn coord(s: &str) -> (i64, i64) {
        let (x, y) = s.split_once(", y=").unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }

    fn radius(&self) -> i64 {
        (self.beacon.0 - self.loc.0).abs() + (self.beacon.1 - self.loc.1).abs()
    }
}
