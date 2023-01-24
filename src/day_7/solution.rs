use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let cwd = parse();
    let part1 = part1(cwd);
    (part1.to_string(), "B".to_string())
}

fn part1(root: Rc<Dir>) -> usize {
    let mut to_visit = vec![Rc::clone(&root)];

    let mut total = 0;
    while let Some(dir) = to_visit.pop() {
        for d in dir.subdir.borrow().values() {
            to_visit.push(Rc::clone(d));
        }

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
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
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
