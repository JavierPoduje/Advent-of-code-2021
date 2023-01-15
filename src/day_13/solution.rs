use std::{cmp::Ordering, str::Chars};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let pairs = parse();
    let part1 = part1(pairs.clone());
    let part2 = part2(pairs);
    (part1.to_string(), part2.to_string())
}

fn part1(pairs: Vec<(Val, Val)>) -> usize {
    let mut sum = 0;
    for (idx, p) in pairs.iter().enumerate() {
        if p.0 < p.1 {
            sum += idx + 1;
        }
    }
    sum
}

fn part2(pairs: Vec<(Val, Val)>) -> usize {
    let mut list = Vec::new();

    for p in pairs.iter() {
        list.push(p.0.clone());
        list.push(p.1.clone());
    }

    let d2 = Val::parse("[[2]]");
    let d6 = Val::parse("[[6]]");

    list.push(d2.clone());
    list.push(d6.clone());

    list.sort();

    let mut answer = 1;
    for (idx, val) in list.iter().enumerate() {
        if *val == d2 || *val == d6 {
            answer *= idx + 1;
        }
    }

    answer
}

fn parse() -> Vec<(Val, Val)> {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_13/input.txt")
        .unwrap()
        .into_iter()
        .filter(|v| !v.is_empty())
        .collect();

    let mut pairs = Vec::new();

    for pair in lines.chunks(2) {
        let left = Val::parse(&pair[0]);
        let right = Val::parse(&pair[1]);
        pairs.push((left, right));
    }

    pairs
}

#[derive(Debug, Eq, PartialEq, Clone, Ord)]
enum Val {
    Num(i32),
    List(Vec<Self>),
}

impl Val {
    pub fn parse(s: &str) -> Val {
        let mut c = s.chars();
        if c.next().unwrap() != '[' {
            panic!("bad input!");
        }
        Self::parse_into(&mut c)
    }

    fn parse_into(c: &mut Chars) -> Self {
        let mut result = Vec::new();
        let mut num = -1;
        while let Some(ch) = c.next() {
            match ch {
                '[' => result.push(Self::parse_into(c)),
                ',' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                        num = -1;
                    }
                }
                ']' => {
                    if num >= 0 {
                        result.push(Self::Num(num));
                    }
                    return Self::List(result);
                }
                '0'..='9' => {
                    if num == -1 {
                        num = (ch as u8 - b'0') as i32;
                    } else {
                        num = (num * 10) + (ch as u8 - b'0') as i32;
                    }
                }
                _ => panic!("bad char '{ch}'"),
            }
        }
        Self::List(result)
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::List(left), Self::List(right)) => {
                let mut idx = 0;
                loop {
                    if left.len() <= idx || right.len() <= idx {
                        if left.len() < right.len() {
                            return Ordering::Less;
                        } else if left.len() == right.len() {
                            return Ordering::Equal;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                    match (&left[idx], &right[idx]) {
                        (Self::Num(l), Self::Num(r)) => {
                            if l < r {
                                return Ordering::Less;
                            } else if l > r {
                                return Ordering::Greater;
                            }
                        }
                        (Self::List(_), Self::Num(r)) => {
                            let check = left[idx].compare(&Self::List(vec![Self::Num(*r)]));
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Self::Num(l), Self::List(_)) => {
                            let check = Self::List(vec![Self::Num(*l)]).compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                        (Self::List(_), Self::List(_)) => {
                            let check = left[idx].compare(&right[idx]);
                            if check != Ordering::Equal {
                                return check;
                            }
                        }
                    }
                    idx += 1;
                }
            }
            _ => panic!("bad input"),
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare(other))
    }
}
