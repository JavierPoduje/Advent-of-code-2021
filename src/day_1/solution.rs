use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> usize {
    let measurements = read_one_per_line::<u32>("./src/day_1/input.txt").unwrap();

    let windowed: Vec<&[u32]> = measurements
        .windows(2)
        .filter(|pair| pair[0] < pair[1])
        .collect();

    windowed.len()
}
