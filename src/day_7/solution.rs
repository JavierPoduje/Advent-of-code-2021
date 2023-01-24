use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let cwd = parse();
    let part1 = part1(cwd.clone());
    let part2 = part2(cwd);
    (part1.to_string(), part2.to_string())
}

fn part2(root: Rc<Dir>) -> usize {
    let total_size = root.get_size();
    let free_space = 70000000 - total_size;
    let space_needed = 30000000 - free_space;

    let mut to_visit = vec![Rc::clone(&root)];

    let mut best = usize::MAX;
    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));
        let size = dir.get_size();
        if size >= space_needed {
            best = best.min(size);
        }
    }

    best
}

fn part1(root: Rc<Dir>) -> usize {
    let mut to_visit = vec![Rc::clone(&root)];

    let mut total = 0;
    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));
        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }
    }

    total
}

fn parse() -> Rc<Dir> {
    let lines: Vec<String> = read_one_per_line::<String>("./src/day_7/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let root = Rc::new(Dir::default());
    let mut cwd = Rc::clone(&root);

    for line in lines {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                dirname => {
                    let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                    cwd = newdir;
                }
            },
            ("dir", dirname) => {
                cwd.subdir.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir {
                        _name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        subdir: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    root
}

#[derive(Default)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}
