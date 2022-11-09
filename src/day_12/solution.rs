use std::collections::{HashMap, HashSet};

use super::super::utils::read_one_per_line::read_one_per_line;

type Nodes = HashMap<String, HashSet<String>>;

pub fn solution() -> (u64, u64) {
    let input = read_one_per_line::<String>("./src/day_12/input.txt").unwrap();
    let nodes = build_nodes(input);

    let valid_paths_for_fst = first_walk(&nodes, "start", &mut HashSet::new());

    let mut hashmap: HashMap<String, u8> =
        nodes
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut hash, (node, _)| {
                hash.insert(node, 0);
                hash
            });
    let valid_paths_for_scd = second_walk(&nodes, "start", &mut hashmap);

    (valid_paths_for_fst, valid_paths_for_scd)
}

fn first_walk(nodes: &Nodes, current: &str, seen: &mut HashSet<String>) -> u64 {
    if current == "end" {
        return 1;
    } else if seen.contains(current) && current.chars().any(|char| char.is_lowercase()) {
        return 0;
    }

    seen.insert(current.to_string());

    let mut valid_paths = 0;
    for other in nodes.get(current).unwrap() {
        valid_paths += first_walk(&nodes, other, seen);
    }

    seen.remove(current);

    valid_paths
}

fn second_walk(nodes: &Nodes, current: &str, seen: &mut HashMap<String, u8>) -> u64 {
    if current == "end" {
        return 1;
    }

    let number_of_visits = seen.get(current).unwrap();
    if (current == "start" && number_of_visits > &1)
        || (number_of_visits >= &2 && current.chars().any(|char| char.is_lowercase()))
    {
        return 0;
    }

    *seen.get_mut(current).unwrap() += 1;

    let mut valid_paths = 0;
    for other in nodes.get(current).unwrap() {
        valid_paths += second_walk(&nodes, other, seen);
    }

    *seen.get_mut(current).unwrap() -= 1;

    valid_paths
}

fn build_nodes(input: Vec<String>) -> Nodes {
    input
        .into_iter()
        .filter(|row| !row.is_empty())
        .fold(HashMap::new(), |mut hash, row| {
            let pair: Vec<&str> = row.split("-").collect::<Vec<&str>>();
            let fst = pair.first().unwrap().to_string();
            let scd = pair.last().unwrap().to_string();

            if !hash.contains_key(&fst) {
                let mut hashset = HashSet::new();
                hashset.insert(scd.clone());
                hash.insert(fst.clone(), hashset);
            } else {
                let hashset = hash.get_mut(&fst).unwrap();
                hashset.insert(scd.clone());
            }

            if !hash.contains_key(&scd) {
                let mut hashset = HashSet::new();
                hashset.insert(fst);
                hash.insert(scd, hashset);
            } else {
                let hashset = hash.get_mut(&scd).unwrap();
                hashset.insert(fst);
            }

            hash
        })
}
