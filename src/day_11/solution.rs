use super::super::utils::read_one_per_line::read_one_per_line;
use std::collections::VecDeque;

pub fn solution() -> (String, String) {
    let rows: Vec<String> = read_one_per_line::<String>("./src/day_11/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.trim().to_owned())
        .collect();

    let mut monkeys = parse(rows);

    let part1 = calc_part1(&mut monkeys.clone());
    let part2 = calc_part2(&mut monkeys);

    (part1.to_string(), part2.to_string())
}

fn calc_part1(monkeys: &mut Vec<Monkey>) -> usize {
    for _ in 0..20 {
        round(monkeys, true);
    }

    let mut monkey_business = monkeys.iter().map(|m| m.count).collect::<Vec<usize>>();
    monkey_business.sort_by(|a, b| b.cmp(a));
    monkey_business[0] * monkey_business[1]
}

fn calc_part2(monkeys: &mut Vec<Monkey>) -> usize {
    for _ in 0..10000 {
        round(monkeys, false);
    }

    let mut monkey_business = monkeys.iter().map(|m| m.count).collect::<Vec<usize>>();
    monkey_business.sort_by(|a, b| b.cmp(a));
    monkey_business[0] * monkey_business[1]
}

fn parse(lines: Vec<String>) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut monkey = Monkey::default();

    for line in lines {
        let words = line.trim().split(' ').collect::<Vec<&str>>();

        match words[0] {
            "Monkey" => monkey = Monkey::default(),
            "Starting" => {
                let (_, strlist) = line.split_once(": ").unwrap();
                monkey.items = strlist.split(", ").map(|w| w.parse().unwrap()).collect();
            }
            "Operation:" => {
                monkey.op = if words[4] == "+" {
                    if words[5] == "old" {
                        Op::AddSelf
                    } else {
                        Op::Add(words[5].parse().unwrap())
                    }
                } else if words[5] == "old" {
                    Op::MulSelf
                } else {
                    Op::Mul(words[5].parse().unwrap())
                };
            }
            "Test:" => monkey.test = words[3].parse().unwrap(),
            "If" => {
                if words[1] == "true:" {
                    monkey.dest.0 = words[5].parse().unwrap();
                } else {
                    monkey.dest.1 = words[5].parse().unwrap();
                    monkeys.push(monkey);
                    monkey = Monkey::default();
                }
            }
            _ => panic!("can't handle '{}' yet", words[0]),
        }
    }

    monkeys
}

fn round(mvec: &mut Vec<Monkey>, part1: bool) {
    let modval: u64 = mvec.iter().map(|m| m.test).product();
    for i in 0..mvec.len() {
        while let Some(item) = mvec[i].items.pop_front() {
            let worry = if part1 {
                mvec[i].op.calc(item) / 3
            } else {
                mvec[i].op.calc(item) % modval
            };

            let dest = if worry % mvec[i].test == 0 {
                mvec[i].dest.0
            } else {
                mvec[i].dest.1
            };
            mvec[dest].items.push_back(worry);
            mvec[i].count += 1;
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    test: u64,
    dest: (usize, usize),
    count: usize,
}

impl Monkey {}

#[derive(Debug, Default, Clone)]
enum Op {
    #[default]
    AddSelf,
    MulSelf,
    Mul(u64),
    Add(u64),
}

impl Op {
    fn calc(&self, val: u64) -> u64 {
        match self {
            Op::AddSelf => val + val,
            Op::MulSelf => val * val,
            Op::Mul(n) => val * *n,
            Op::Add(n) => val + *n,
        }
    }
}
