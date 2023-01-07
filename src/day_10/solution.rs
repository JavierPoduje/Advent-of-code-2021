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

    let mut screen: Vec<[char; 40]> = Vec::new();
    screen.push(['.'; 40]);

    for (row_number, (command, v)) in rows.iter().enumerate() {
        match command.as_str() {
            "noop" => {
                cycles += 1;
                if is_interesting_signal_cycle(cycles) {
                    signal_strength.push(register * cycles);
                }

                let idx = (cycles - 1) % 40;

                if idx - 1 <= register && register <= idx + 1 {
                    let last_row = screen.last_mut().unwrap();
                    *last_row.get_mut(idx as usize).unwrap() = '#';
                }

                if should_add_row(cycles, row_number, rows.len()) {
                    screen.push(['.'; 40]);
                }
            }
            "addx" => {
                for _ in 0..2 {
                    cycles += 1;
                    if is_interesting_signal_cycle(cycles) {
                        signal_strength.push(register * cycles);
                    }

                    let idx = (cycles - 1) % 40;

                    if idx - 1 <= register && register <= idx + 1 {
                        let last_row = screen.last_mut().unwrap();
                        *last_row.get_mut(idx as usize).unwrap() = '#';
                    }

                    if should_add_row(cycles, row_number, rows.len()) {
                        screen.push(['.'; 40]);
                    }
                }
                register += v.unwrap();
            }
            _ => unreachable!(),
        }
    }

    let part2 = screen.iter().fold(String::from("\n"), |mut acc, row| {
        acc.push_str(&row.map(|char| char.to_string()).join(""));
        acc.push_str("\n");
        acc
    });

    let part1: i64 = signal_strength.iter().sum();

    (part1.to_string(), part2)
}

fn should_add_row(cycles: i64, row_number: usize, total_rows: usize) -> bool {
    row_number < total_rows - 1 && (cycles - 1) > 0 && (cycles - 1) % 40 == 39
}

fn is_interesting_signal_cycle(cycles: i64) -> bool {
    if cycles < 20 {
        return false;
    }

    (cycles - 20) % 40 == 0
}
