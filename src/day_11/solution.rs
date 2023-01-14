use super::super::utils::read_one_per_line::read_one_per_line;
use std::collections::VecDeque;

pub fn solution() -> (String, String) {
    let rows: Vec<String> = read_one_per_line::<String>("./src/day_11/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.trim().to_owned())
        .collect();

    let mut monkeys: Vec<Monkey> = build_initial_monkeys(&rows);

    let mut idx: usize = 0;
    let mut rounds = 0;
    let number_of_rounds = 10000;
    let mut part1 = 0;
    let modval: u128 = monkeys.iter().map(|m| m.test_value).product();

    while rounds < number_of_rounds {
        while monkeys.get(idx).unwrap().items.len() > 0 {
            // pop front item
            let mut item = monkeys.get_mut(idx).unwrap().items.pop_front().unwrap();
            let mut raw_item = monkeys.get_mut(idx).unwrap().raw_items.pop_front().unwrap();

            // add inspection
            monkeys.get_mut(idx).unwrap().add_inspection();

            // operate (and divide by three the non-raw item)
            item = monkeys.get(idx).unwrap().call_op(item) / 3;

            raw_item = monkeys.get(idx).unwrap().call_op(raw_item / modval);

            // throw item to another monkey
            if monkeys.get(idx).unwrap().call_test(item) {
                let other_monkey_idx = monkeys.get(idx).unwrap().on_true;
                monkeys.get_mut(other_monkey_idx).unwrap().add_item(item);
            } else {
                let other_monkey_idx = monkeys.get(idx).unwrap().on_false;
                monkeys.get_mut(other_monkey_idx).unwrap().add_item(item);
            }

            // throw item to another monkey
            if monkeys.get(idx).unwrap().call_test(raw_item) {
                let other_monkey_idx = monkeys.get(idx).unwrap().on_true;
                monkeys
                    .get_mut(other_monkey_idx)
                    .unwrap()
                    .add_item(raw_item);
            } else {
                let other_monkey_idx = monkeys.get(idx).unwrap().on_false;
                monkeys
                    .get_mut(other_monkey_idx)
                    .unwrap()
                    .add_item(raw_item);
            }
        }

        // at this point, the monkey was already emptied
        idx += 1;
        if idx == monkeys.len() {
            idx = 0;
            rounds += 1;
        }

        if rounds == 20 {
            let mut monkeys_inspections: Vec<u128> =
                monkeys.iter().map(|monkey| monkey.inspections).collect();
            monkeys_inspections.sort();
            monkeys_inspections.reverse();
            let inspections: Vec<&u128> = monkeys_inspections.iter().take(2).collect();
            part1 = inspections[0] * inspections[1];
        }
    }

    let mut monkeys_inspections: Vec<u128> =
        monkeys.iter().map(|monkey| monkey.inspections).collect();
    monkeys_inspections.sort();
    monkeys_inspections.reverse();
    let inspections: Vec<&u128> = monkeys_inspections.iter().take(2).collect();
    let part2 = inspections[0] * inspections[1];

    (part1.to_string(), part2.to_string())
}

struct Monkey {
    items: VecDeque<u128>,
    raw_items: VecDeque<u128>,
    op: Box<dyn Fn(u128) -> Box<dyn Fn(u128) -> u128>>,
    op_value: u128,
    inspections: u128,
    test: Box<dyn Fn(u128) -> Box<dyn Fn(u128) -> bool>>,
    test_value: u128,
    on_true: usize,
    on_false: usize,

    use_new: bool,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("op_value", &self.op_value)
            .field("test_value", &self.test_value)
            .field("on_true", &self.on_true)
            .field("on_false", &self.on_false)
            .field("use_new", &self.use_new)
            .finish()
    }
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
            raw_items: VecDeque::new(),
            inspections: 0,
            op: Box::new(|a| Box::new(move |b| a + b)),
            op_value: 1,
            test: Box::new(|a| Box::new(move |b| a % b == 0)),
            test_value: 1,
            on_true: 1,
            on_false: 1,
            use_new: false,
        }
    }

    pub fn set_use_new(&mut self, use_new: bool) {
        self.use_new = use_new;
    }

    pub fn set_items(&mut self, items: VecDeque<u128>) {
        self.items = items.clone();
        self.raw_items = items;
    }

    pub fn set_op(&mut self, op: Box<dyn Fn(u128) -> Box<dyn Fn(u128) -> u128>>, value: u128) {
        self.op = op;
        self.op_value = value;
    }

    pub fn call_op(&self, b: u128) -> u128 {
        if self.use_new {
            return (self.op)(b)(b);
        }
        (self.op)(self.op_value)(b)
    }

    pub fn call_test(&self, value_to_test: u128) -> bool {
        (self.test)(value_to_test)(self.test_value)
    }

    pub fn set_test(
        &mut self,
        test: Box<dyn Fn(u128) -> Box<dyn Fn(u128) -> bool>>,
        test_value: u128,
    ) {
        self.test = test;
        self.test_value = test_value;
    }
    pub fn set_true(&mut self, on_true: usize) {
        self.on_true = on_true;
    }
    pub fn set_false(&mut self, on_false: usize) {
        self.on_false = on_false;
    }
    pub fn add_item(&mut self, item: u128) {
        self.items.push_back(item);
        self.raw_items.push_back(item);
    }

    pub fn add_inspection(&mut self) {
        self.inspections += 1;
    }
}

fn build_initial_monkeys(rows: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for row in rows {
        if row.starts_with("Monkey") {
            monkeys.push(Monkey::new());
        } else if row.starts_with("Operation") {
            let def = row
                .split(':')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .split('=')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .split(' ')
                .filter(|value| !value.is_empty())
                .collect::<Vec<&str>>();

            let _left = def[0];
            let operator = def[1];
            let right = def[2];

            match operator {
                "*" => {
                    if let Some(monkey) = monkeys.last_mut() {
                        let op_value = match right {
                            "old" => {
                                monkey.set_use_new(true);
                                1
                            }
                            "new" => {
                                monkey.set_use_new(true);
                                1
                            }
                            _ => right.parse::<u128>().unwrap(),
                        };
                        monkey.set_op(Box::new(|a| Box::new(move |b| a * b)), op_value);
                    }
                }
                "+" => {
                    if let Some(monkey) = monkeys.last_mut() {
                        let op_value = match right {
                            "old" => {
                                monkey.set_use_new(true);
                                1
                            }
                            "new" => {
                                monkey.set_use_new(true);
                                1
                            }
                            _ => right.parse::<u128>().unwrap(),
                        };
                        monkey.set_op(Box::new(|a| Box::new(move |b| a + b)), op_value);
                    }
                }
                "-" => {
                    if let Some(monkey) = monkeys.last_mut() {
                        let op_value = match right {
                            "old" => {
                                monkey.set_use_new(true);
                                1
                            }
                            "new" => {
                                monkey.set_use_new(true);
                                1
                            }
                            _ => right.parse::<u128>().unwrap(),
                        };
                        monkey.set_op(Box::new(|a| Box::new(move |b| a - b)), op_value);
                    }
                }
                "/" => {
                    if let Some(monkey) = monkeys.last_mut() {
                        let op_value = match right {
                            "old" => {
                                monkey.set_use_new(true);
                                1
                            }
                            "new" => {
                                monkey.set_use_new(true);
                                1
                            }
                            _ => right.parse::<u128>().unwrap(),
                        };
                        monkey.set_op(Box::new(|a| Box::new(move |b| a / b)), op_value);
                    }
                }
                "" => {}
                _ => {
                    unreachable!();
                }
            }
        } else if row.starts_with("Test") {
            let divisible_by = row
                .split(':')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .trim()
                .parse::<u128>()
                .unwrap();

            if let Some(monkey) = monkeys.last_mut() {
                monkey.set_test(Box::new(|a| Box::new(move |b| a % b == 0)), divisible_by);
            }
        } else if row.starts_with("If true") {
            let monkey_to_throw_idx = row
                .split(':')
                .last()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            if let Some(monkey) = monkeys.last_mut() {
                monkey.set_true(monkey_to_throw_idx);
            }
        } else if row.starts_with("If false") {
            let monkey_to_throw_idx = row
                .split(':')
                .last()
                .unwrap()
                .split(' ')
                .last()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            if let Some(monkey) = monkeys.last_mut() {
                monkey.set_false(monkey_to_throw_idx);
            }
        } else if row.starts_with("Starting items") {
            let items: Vec<u128> = row
                .split(':')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .split(',')
                .map(|value: &str| {
                    let val = value.trim().parse::<u128>().unwrap();
                    val
                })
                .collect();
            if let Some(monkey) = monkeys.last_mut() {
                monkey.set_items(VecDeque::from(items));
            }
        } else {
            unreachable!();
        }
    }
    monkeys
}
