use std::collections::{HashMap, HashSet};

use super::super::utils::read_one_per_line::read_one_per_line;

const THRESHOLD: u64 = 100000;

pub fn solution() -> (String, String) {
    let rows: Vec<String> = read_one_per_line::<String>("./src/day_7/input.txt")
        .unwrap()
        .into_iter()
        .filter(|row| !row.is_empty())
        .collect();

    let mut system = build_system(rows);
    let system = weigh_system(&mut system);

    let part1 = calculate_part1(&system);

    (part1, "B".to_string())
}

fn calculate_part1(system: &HashMap<String, Dir>) -> String {
    system
        .iter()
        .fold(0, |acc, (_, dir)| {
            match dir.size <= THRESHOLD {
                true => acc + dir.size,
                _ => acc,
            }
        })
        .to_string()
}

fn build_system(rows: Vec<String>) -> HashMap<String, Dir> {
    let mut system = HashMap::new();

    system.insert("/".to_string(), Dir::new("/".to_string()));
    let mut dirs_stack: Vec<String> = vec!["/".to_string()];

    for row in rows {
        let line: Vec<&str> = row.split(" ").collect();

        match line[0] {
            "$" => match command(line[1]) {
                Command::CD => match line[2] {
                    "/" => {}
                    ".." => {
                        dirs_stack.pop();
                    }
                    dir => {
                        dirs_stack.push(dir.to_string());
                    }
                },
                Command::LS => {}
            },
            "dir" => {
                let dir_name = line[1].to_string();
                let current_directory_name = dirs_stack.last().unwrap();

                system.insert(dir_name.clone(), Dir::new(dir_name.clone()));

                let current_directory = system.get_mut(current_directory_name).unwrap();
                current_directory.add_dir(dir_name);
            }
            size => {
                let current_directory_name = dirs_stack.last().unwrap();
                let current_directory = system.get_mut(current_directory_name).unwrap();
                current_directory.add_file(size.to_string(), line[1].to_string());
            }
        }
    }

    system
}

fn weigh_system(system: &mut HashMap<String, Dir>) -> &mut HashMap<String, Dir> {
    let mut weight_by_dir_name = HashMap::<String, u64>::new();
    for (dir_name, _) in system.iter() {
        let weight = weight_directory(&system, dir_name);
        weight_by_dir_name.insert(dir_name.to_string(), weight);
    }

    for (dir_name, weight) in weight_by_dir_name {
        let mut dir = system.get_mut(&dir_name).unwrap();
        dir.size = weight;
    }

    system
}

fn weight_directory(system: &HashMap<String, Dir>, dir_name: &String) -> u64 {
    let curr_dir = system.get(dir_name).unwrap();
    let mut weight: u64 = curr_dir
        .clone()
        .files
        .iter()
        .fold(0, |acc, file| acc + file.size);

    for child_dir_name in &curr_dir.dirs {
        let child_dir = system.get(child_dir_name).unwrap();
        if child_dir.size > 0 {
            weight += child_dir.size;
        } else {
            weight += weight_directory(system, child_dir_name);
        }
    }

    weight
}

enum Command {
    LS,
    CD,
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    files: Vec<File>,
    size: u64,
    dirs: HashSet<String>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u64,
}

impl Dir {
    pub fn new(dir_name: String) -> Self {
        Self {
            name: dir_name,
            files: Vec::new(),
            dirs: HashSet::new(),
            size: 0,
        }
    }

    pub fn add_dir(&mut self, dir_name: String) {
        self.dirs.insert(dir_name);
    }

    pub fn add_file(&mut self, file_size: String, file_name: String) {
        self.files
            .push(File::new(file_name, file_size.parse::<u64>().unwrap()));
    }
}

fn command(command_name: &str) -> Command {
    match command_name {
        "cd" => Command::CD,
        "ls" => Command::LS,
        _ => unreachable!(),
    }
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}
