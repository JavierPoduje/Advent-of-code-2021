use std::fs;

pub fn solution() -> i32 {
    // read input file
    let input = fs::read_to_string("./src/day_1/input.txt").unwrap();

    // split by line jump
    let measurements: Vec<&str> = input.split("\n").collect();

    // declare
    let mut prev = f64::INFINITY;
    let mut incs = 0;

    // iterate through all measurements
    for curr in measurements {
        // handle empty strings
        if curr.is_empty() {
            continue;
        }

        let curr_num = curr.parse().unwrap();

        if curr_num > prev {
            incs += 1;
        }

        prev = curr_num;
    }

    // return the increments
    incs
}
