use std::collections::HashMap;

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let tree = parse();
    let part1 = part1(tree.clone());
    let part2 = part2(tree);
    (part1.to_string(), part2.to_string())
}

fn part1(tree: HashMap<String, Monkey>) -> i64 {
    eval(&"root".to_string(), &tree)
}

fn part2(tree: HashMap<String, Monkey>) -> i64 {
    let root = "root".to_string();
    let path = find_human(&root, &tree).unwrap();
    let path = path.iter().rev().copied().collect::<Vec<_>>();

    let (left, right) = match tree.get(&root).unwrap() {
        Monkey::Value(_) => panic!("root monkey has no data"),
        Monkey::Binary(left, _, right) => (left, right),
    };

    let correct_val = if left == path[1] {
        eval(right, &tree)
    } else {
        eval(left, &tree)
    };

    find_adjustment(&path, 1, &tree, correct_val)
}

fn find_adjustment(
    path: &Vec<&String>,
    index: usize,
    tree: &HashMap<String, Monkey>,
    cv: i64,
) -> i64 {
    match tree.get(path[index]).unwrap() {
        Monkey::Value(_) => cv,
        Monkey::Binary(l, op, r) => {
            let left = eval(l, tree);
            let right = eval(r, tree);
            let new_cv = if l == path[index + 1] {
                match op {
                    Operation::Addition => cv - right,
                    Operation::Subtraction => cv + right,
                    Operation::Multiplication => cv / right,
                    Operation::Division => cv * right,
                }
            } else {
                match op {
                    Operation::Addition => cv - left,
                    Operation::Subtraction => left - cv,
                    Operation::Multiplication => cv / left,
                    Operation::Division => left / cv,
                }
            };
            find_adjustment(path, index + 1, tree, new_cv)
        }
    }
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

fn find_human<'a>(loc: &'a String, tree: &'a HashMap<String, Monkey>) -> Option<Vec<&'a String>> {
    if loc == "humn" {
        return Some(vec![loc]);
    }

    if let Some(Monkey::Binary(l, _, r)) = tree.get(loc) {
        if let Some(mut vec) = find_human(l, tree) {
            vec.push(loc);
            return Some(vec);
        }
        if let Some(mut vec) = find_human(r, tree) {
            vec.push(loc);
            return Some(vec);
        }
    }

    None
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
                Monkey::Binary(
                    expr[0].to_string(),
                    Operation::parse(expr[1].chars().next().unwrap()),
                    expr[2].to_string(),
                ),
            );
        }
    }

    tree
}

#[derive(Debug, Clone)]
enum Monkey {
    Value(i64),
    Binary(String, Operation, String),
}

#[derive(Debug, Clone)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
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
