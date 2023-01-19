use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let tree = parse();
    let part1 = part1(tree);
    (part1.to_string(), "B".to_string())
}

fn part1(tree: HashMap<String, Monkey>) -> i64 {
    eval(&"root".to_string(), &tree)
}

fn eval(start: &String, tree: &HashMap<String, Monkey>) -> i64 {
    match tree.get(start).unwrap() {
        Monkey::Value(v) => *v,
        Monkey::Binary(l, op, r) => {
            let l = eval(l, tree);
            let r = eval(r, tree);
            match op {
                Operation::Addition => l + r,
                Operation::Subtraction => l - r,
                Operation::Multiplication => l * r,
                Operation::Division => l / r,
            }
        }
    }
}

fn parse() -> HashMap<String, Monkey> {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_21/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let mut tree = HashMap::new();

    for line in lines {
        let (left, right) = line.split_once(": ").unwrap();
        if let Ok(num) = right.parse::<i64>() {
            tree.insert(left.to_string(), Monkey::Value(num));
        } else {
            let expr = right.split_whitespace().collect::<Vec<_>>();
            tree.insert(
                left.to_string(),
                Monkey::Binary(expr[0].to_string(),
                Operation::parse(
                    expr[1].chars().next().unwrap()),
                    expr[2].to_string(),
                )
            );
        }
    }

    println!("{tree:?}");

    tree
}

#[derive(Debug)]
enum Monkey {
    Value(i64),
    Binary(String, Operation, String),
}

#[derive(Debug)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division
}

impl Operation {
    fn parse(ch: char) -> Self {
        match ch {
            '+' => Self::Addition,
            '-' => Self::Subtraction,
            '*' => Self::Multiplication,
            '/' => Self::Division,
            _ => panic!("not an operation: '{ch}'"),
        }
    }
}
