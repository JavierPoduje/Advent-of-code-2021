use std::collections::HashSet;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let rows: Vec<String> = read_one_per_line::<String>("./src/day_4/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let elf_pairs: Vec<[Elf; 2]> = rows
        .iter()
        .map(|row| {
            let pair: Vec<&str> = row.split(",").collect();
            [
                Elf::new(pair.first().unwrap()),
                Elf::new(pair.last().unwrap()),
            ]
        })
        .collect();

    let mut part_1 = 0;
    for pair in elf_pairs {
        if redundant(pair) {
            part_1 += 1;
        }
    }

    (part_1, 0)
}

fn redundant([fst, scd]: [Elf; 2]) -> bool {
    let fst_contains_scd = scd
        .sections
        .iter()
        .all(|section| fst.sections.contains(section));
    let scd_contains_fst = fst
        .sections
        .iter()
        .all(|section| scd.sections.contains(section));
    fst_contains_scd || scd_contains_fst
}

#[derive(Debug)]
struct Elf {
    sections: HashSet<i32>,
}

impl Elf {
    pub fn new(row: &str) -> Self {
        let pair: Vec<&str> = row.split("-").collect();
        let fst = pair.first().unwrap().parse::<i32>().unwrap();
        let scd = pair.last().unwrap().parse::<i32>().unwrap();

        let mut sections: HashSet<i32> = HashSet::new();
        for value in fst..=scd {
            sections.insert(value);
        }

        Self { sections }
    }
}
