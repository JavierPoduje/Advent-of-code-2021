use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let rows: Vec<_> = read_one_per_line::<String>("./src/day_2/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            let splitted: Vec<&str> = row.as_str().split(" ").collect();
            (
                Thing::new(splitted.first().unwrap()),
                Thing::new(splitted.last().unwrap()),
            )
        })
        .collect();

    let total_score = rows.into_iter().fold(0, |acc, row| acc + score(row));

    (total_score, 0)
}

fn score((fst, scd): (Thing, Thing)) -> u64 {
    let bout_score = scd.bout(fst);
    let value = scd.to_value();
    bout_score + value
}

enum Thing {
    Rock,
    Papper,
    Scissors,
}

impl Thing {
    pub fn new(str: &str) -> Self {
        match str {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Papper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn bout(&self, other: Self) -> u64 {
        match self {
            Self::Rock => match other {
                Self::Rock => 3,
                Thing::Papper => 0,
                Thing::Scissors => 6,
            },
            Thing::Papper => match other {
                Self::Rock => 6,
                Thing::Papper => 3,
                Thing::Scissors => 0,
            },
            Thing::Scissors => match other {
                Self::Rock => 0,
                Thing::Papper => 6,
                Thing::Scissors => 3,
            },
        }
    }

    pub fn to_value(&self) -> u64 {
        match self {
            Thing::Rock => 1,
            Thing::Papper => 2,
            Thing::Scissors => 3,
        }
    }
}
