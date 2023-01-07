use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let rows: Vec<(String, Option<i64>)> = read_one_per_line::<String>("./src/day_10/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| {
            let splitted: Vec<&str> = row.split(" ").collect();
            match splitted.len() {
                1 => (splitted[0].to_string(), None),
                2 => (
                    splitted[0].to_string(),
                    Some(splitted[1].parse::<i64>().unwrap()),
                ),
                _ => unreachable!(),
            }
        })
        .collect();

    let mut cycles: i64 = 0;
    let mut register: i64 = 1;
    let mut signal_strength: Vec<i64> = Vec::new();

    for (command, v) in rows {
        match command.as_str() {
            "noop" => {
                cycles += 1;
                if is_interesting_signal_cycle(cycles) {
                    signal_strength.push(register * cycles);
                }
            }
            "addx" => {
                for _ in 0..2 {
                    cycles += 1;
                    if is_interesting_signal_cycle(cycles) {
                        signal_strength.push(register * cycles);
                    }
                }
                register += v.unwrap();
            }
            _ => unreachable!(),
        }
    }

    let part1: i64 = signal_strength.iter().sum();

    (part1.to_string(), "B".to_string())
}

fn is_interesting_signal_cycle(cycles: i64) -> bool {
    if cycles < 20 {
        return false;
    }

    (cycles - 20) % 40 == 0
}
