use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let rows: Vec<_> = read_one_per_line::<String>("./src/day_2/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let mut rows_for_fst: Vec<(Thing, Thing)> = Vec::new();
    let mut rows_for_scd: Vec<(Thing, Thing)> = Vec::new();

    for row in rows {
        let splitted: Vec<&str> = row.as_str().split(" ").collect();

        let fst = splitted.first().unwrap();
        let scd = splitted.last().unwrap();

        rows_for_fst.push((Thing::new(fst), Thing::new(scd)));
        rows_for_scd.push((Thing::new(fst), Thing::new_calculated(fst, scd)));
    }

    let fst_score = rows_for_fst
        .clone()
        .into_iter()
        .fold(0, |acc, row| acc + score(row));
    let scd_score = rows_for_scd
        .into_iter()
        .fold(0, |acc, row| acc + score(row));

    (fst_score, scd_score)
}

fn score((fst, scd): (Thing, Thing)) -> u64 {
    let bout_score = scd.bout(fst);
    let value = scd.to_value();
    bout_score + value
}

#[derive(Clone, Debug)]
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

    pub fn new_calculated(fst: &str, scd: &str) -> Self {
        match scd {
            "X" => Thing::lose(fst), // lose
            "Y" => Thing::draw(fst), // draw
            "Z" => Thing::win(fst),  // win
            _ => unreachable!(),
        }
    }

    fn win(char: &str) -> Self {
        match char {
            "A" => Self::Papper,
            "B" => Self::Scissors,
            "C" => Self::Rock,
            _ => unreachable!(),
        }
    }

    fn lose(char: &str) -> Self {
        match char {
            "A" => Self::Scissors,
            "B" => Self::Rock,
            "C" => Self::Papper,
            _ => unreachable!(),
        }
    }

    fn draw(char: &str) -> Self {
        match char {
            "A" => Self::Rock,
            "B" => Self::Papper,
            "C" => Self::Scissors,
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
