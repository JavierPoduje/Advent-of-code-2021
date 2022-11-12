use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
    let input: Vec<String> = read_one_per_line::<String>("./src/day_14/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    // get the sequence
    let raw_seq = input.first().unwrap().to_string();
    let polymer: Vec<&str> = raw_seq
        .split("")
        .filter(|value| !value.is_empty())
        .collect::<Vec<&str>>()
        .to_vec();

    // build the hashmap
    let hashmap: HashMap<String, String> = input
        .into_iter()
        .enumerate()
        .filter(|&(idx, _)| idx > 0)
        .fold(HashMap::new(), |mut acc, (_, line)| {
            let pair: Vec<&str> = line.split(" -> ").collect();
            acc.insert(pair[0].to_string(), pair[1].to_string());
            acc
        });

    let part1 = calculate(extend_polymerization(&polymer, 10, &hashmap));
    let part2 = calculate(extend_polymerization(&polymer, 40, &hashmap));

    (part1, part2)
}

fn calculate(extended: Vec<String>) -> u64 {
    let times_by_letter =
        extended
            .into_iter()
            .fold(HashMap::<String, u64>::new(), |mut acc, letter| {
                match acc.get(&*letter) {
                    Some(value) => acc.insert(letter.clone(), value + 1),
                    None => acc.insert(letter.clone(), 1),
                };
                acc
            });

    let mut max_value: Option<u64> = None;
    let mut min_value: Option<u64> = None;

    for (_, value) in times_by_letter.into_iter() {
        match max_value {
            Some(current) => max_value = Some(std::cmp::max(current, value)),
            None => max_value = Some(value),
        }
        match min_value {
            Some(current) => min_value = Some(std::cmp::min(current, value)),
            None => min_value = Some(value),
        }
    }

    max_value.unwrap() - min_value.unwrap()
}

fn extend_polymerization(
    polymer: &Vec<&str>,
    times: usize,
    hashmap: &HashMap<String, String>,
) -> Vec<String> {
    let mut seq = polymer.clone();

    for _ in 0..times {
        seq = seq
            .windows(2)
            .enumerate()
            .fold(Vec::<&str>::new(), |mut acc, (idx, chars)| {
                let fst = chars[0];
                let scd = chars[1];
                let key = format!("{}{}", fst.clone(), scd.clone());

                match hashmap.get(&key) {
                    Some(middle) => {
                        if idx == 0 {
                            acc.push(fst);
                        }
                        acc.push(middle);
                        acc.push(scd);
                    }
                    None => {
                        if idx == 0 {
                            acc.push(fst);
                        }
                        acc.push(scd);
                    }
                }

                acc
            });
    }

    seq.into_iter().map(|value| value.to_string()).collect()
}

#[test]
fn one_time() {
    let tuples = [
        ("NC".to_string(), "B".to_string()),
        ("CC".to_string(), "N".to_string()),
        ("NH".to_string(), "C".to_string()),
        ("CH".to_string(), "B".to_string()),
        ("CB".to_string(), "H".to_string()),
        ("NB".to_string(), "B".to_string()),
        ("BC".to_string(), "B".to_string()),
        ("HN".to_string(), "C".to_string()),
        ("NN".to_string(), "C".to_string()),
        ("HB".to_string(), "C".to_string()),
        ("BB".to_string(), "N".to_string()),
        ("CN".to_string(), "C".to_string()),
        ("BN".to_string(), "B".to_string()),
        ("BH".to_string(), "H".to_string()),
        ("HC".to_string(), "B".to_string()),
        ("HH".to_string(), "N".to_string()),
    ];
    let polymer = Vec::from(["N", "N", "C", "B"]);
    let times: usize = 1;
    let hashmap: HashMap<String, String> = tuples.into_iter().collect();

    let res = extend_polymerization(&polymer, times, &hashmap);
    assert_eq!(res, ["N", "C", "N", "B", "C", "H", "B"]);
}

#[test]
fn two_time() {
    let tuples = [
        ("NC".to_string(), "B".to_string()),
        ("CC".to_string(), "N".to_string()),
        ("NH".to_string(), "C".to_string()),
        ("CH".to_string(), "B".to_string()),
        ("CB".to_string(), "H".to_string()),
        ("NB".to_string(), "B".to_string()),
        ("BC".to_string(), "B".to_string()),
        ("HN".to_string(), "C".to_string()),
        ("NN".to_string(), "C".to_string()),
        ("HB".to_string(), "C".to_string()),
        ("BB".to_string(), "N".to_string()),
        ("CN".to_string(), "C".to_string()),
        ("BN".to_string(), "B".to_string()),
        ("BH".to_string(), "H".to_string()),
        ("HC".to_string(), "B".to_string()),
        ("HH".to_string(), "N".to_string()),
    ];
    let polymer = Vec::from(["N", "N", "C", "B"]);
    let times: usize = 2;
    let hashmap: HashMap<String, String> = tuples.into_iter().collect();

    let res = extend_polymerization(&polymer, times, &hashmap);
    assert_eq!(
        res,
        ["N", "B", "C", "C", "N", "B", "B", "B", "C", "B", "H", "C", "B"]
    );
}

#[test]
fn three_time() {
    let tuples = [
        ("NC".to_string(), "B".to_string()),
        ("CC".to_string(), "N".to_string()),
        ("NH".to_string(), "C".to_string()),
        ("CH".to_string(), "B".to_string()),
        ("CB".to_string(), "H".to_string()),
        ("NB".to_string(), "B".to_string()),
        ("BC".to_string(), "B".to_string()),
        ("HN".to_string(), "C".to_string()),
        ("NN".to_string(), "C".to_string()),
        ("HB".to_string(), "C".to_string()),
        ("BB".to_string(), "N".to_string()),
        ("CN".to_string(), "C".to_string()),
        ("BN".to_string(), "B".to_string()),
        ("BH".to_string(), "H".to_string()),
        ("HC".to_string(), "B".to_string()),
        ("HH".to_string(), "N".to_string()),
    ];
    let polymer = Vec::from(["N", "N", "C", "B"]);
    let times: usize = 3;
    let hashmap: HashMap<String, String> = tuples.into_iter().collect();

    let res = extend_polymerization(&polymer, times, &hashmap);
    assert_eq!(
        res,
        [
            "N", "B", "B", "B", "C", "N", "C", "C", "N", "B", "B", "N", "B", "N", "B", "B", "C",
            "H", "B", "H", "H", "B", "C", "H", "B"
        ]
    );
}

#[test]
fn four_time() {
    let tuples = [
        ("NC".to_string(), "B".to_string()),
        ("CC".to_string(), "N".to_string()),
        ("NH".to_string(), "C".to_string()),
        ("CH".to_string(), "B".to_string()),
        ("CB".to_string(), "H".to_string()),
        ("NB".to_string(), "B".to_string()),
        ("BC".to_string(), "B".to_string()),
        ("HN".to_string(), "C".to_string()),
        ("NN".to_string(), "C".to_string()),
        ("HB".to_string(), "C".to_string()),
        ("BB".to_string(), "N".to_string()),
        ("CN".to_string(), "C".to_string()),
        ("BN".to_string(), "B".to_string()),
        ("BH".to_string(), "H".to_string()),
        ("HC".to_string(), "B".to_string()),
        ("HH".to_string(), "N".to_string()),
    ];
    let polymer = Vec::from(["N", "N", "C", "B"]);
    let times: usize = 4;
    let hashmap: HashMap<String, String> = tuples.into_iter().collect();

    let res = extend_polymerization(&polymer, times, &hashmap);
    assert_eq!(
        res,
        [
            "N", "B", "B", "N", "B", "N", "B", "B", "C", "C", "N", "B", "C", "N", "C", "C", "N",
            "B", "B", "N", "B", "B", "N", "B", "B", "B", "N", "B", "B", "N", "B", "B", "C", "B",
            "H", "C", "B", "H", "H", "N", "H", "C", "B", "B", "C", "B", "H", "C", "B"
        ]
    );
}
