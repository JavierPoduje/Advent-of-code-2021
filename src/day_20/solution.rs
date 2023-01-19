use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let data = parse();
    let part1 = part1(data.clone());
    let part2 = part2(data);
    (part1.to_string(), part2.to_string())
}

fn part1(data: Vec<(i64, i64)>) -> i64 {
    decrypt(&data, 1, 1)
}

fn part2(data: Vec<(i64, i64)>) -> i64 {
    decrypt(&data, 10, 811589153)
}

fn parse() -> Vec<(i64, i64)> {
    read_one_per_line::<String>("./src/day_20/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|v| v.parse::<i64>().unwrap())
        .enumerate()
        .map(|n| (n.0 as i64, n.1))
        .collect()
}

fn decrypt(data: &Vec<(i64, i64)>, rounds: usize, key: i64) -> i64 {
    let data = data
        .iter()
        .map(|(index, value)| (*index, value * key))
        .collect::<Vec<_>>();

    let mut result = data.clone();

    let len = result.len() as i64 - 1;

    for _ in 0..rounds {
        for d in &data {
            let pos = result.iter().position(|n| n == d).unwrap() as i64;

            let mut new_pos = (pos + d.1) % len;

            if new_pos < 0 {
                new_pos += len;
            }

            if new_pos >= len {
                new_pos -= len;
            }

            let val = result.remove(pos as usize);
            result.insert(new_pos as usize, val);
        }
    }

    let zero = result.iter().position(|p| p.1 == 0).unwrap();

    result[(zero + 1000) % result.len()].1
        + result[(zero + 2000) % result.len()].1
        + result[(zero + 3000) % result.len()].1
}
