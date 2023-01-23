use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let nums = parse();
    let part1 = part1(nums);
    (part1.to_string(), "b".to_string())
}

fn part1(nums: Vec<String>) -> String {
    let sum = nums.iter().map(|s| to_i64(s)).sum();
    to_snafu(sum)
}

fn to_i64(s: &String) -> i64 {
    let mut result = 0;
    for c in s.chars() {
        match c {
            '0'..='2' => result = (result * 5) + (c as u8 - b'0') as i64,
            '-' => result = result * 5 - 1,
            '=' => result = result * 5 - 2,
            _ => panic!("invalid char '{c}'"),
        }
    }
    result
}

fn to_snafu(mut num: i64) -> String {
    let mut result = String::new();

    loop {
        let n = num % 5;
        match n {
            0..=2 => {
                result.push((n as u8 + b'0') as char);
                num /= 5;
            }
            3 => {
                result.push('=');
                num = (num + 2) / 5;
            }
            4 => {
                result.push('-');
                num = (num + 1) / 5;
            }
            _ => panic!("math is broken"),
        }

        if num == 0 {
            break;
        }
    }

    result.chars().rev().collect()
}

fn parse() -> Vec<String> {
    read_one_per_line::<String>("./src/day_25/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect()
}
