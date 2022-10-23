use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (i32, i32) {
    let part1 = part1();
    let part2 = part2();

    (part1, part2)
}

pub fn part2() -> i32 {
    let commands = read_one_per_line::<String>("./src/day_2/part2.txt").unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for command in commands {
        if command == "" {
            continue
        }

        let splitted_command: Vec<&str> = command.split::<&str>(" ").collect();
        let dimension = splitted_command[0];
        let units: i32 = splitted_command[1].parse().unwrap();

        match dimension {
            "forward" => {
                x += units;
                y += aim * units;
            }
            "down" => {
                aim += units;
            }
            "up" => {
                aim -= units;
            }
            _ => unreachable!(),
        }
    }

    x * y
}

pub fn part1() -> i32 {
    let commands = read_one_per_line::<String>("./src/day_2/part1.txt").unwrap();

    let mut x = 0;
    let mut y = 0;

    for command in commands {
        if command == "" {
            continue
        }

        let splitted_command: Vec<&str> = command.split::<&str>(" ").collect();
        let dimension = splitted_command[0];
        let units: i32 = splitted_command[1].parse().unwrap();

        match dimension {
            "forward" => {
                x += units;
            }
            "down" => {
                y += units;
            }
            "up" => {
                y -= units;
            }
            _ => unreachable!(),
        }
    }

    x * y
}
