use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (String, String) {
    let rows = read_one_per_line::<String>("./src/day_1/input.txt").unwrap();

    let mut calories = Vec::<u64>::new();
    let mut calorie: u64 = 0;

    for row in rows {
        match row.as_str() {
            "" => {
                calories.push(calorie);
                calorie = 0;
            }
            _ => {
                let current: u64 = row.parse().unwrap();
                calorie += current;
            }
        }
    }

    let part1: u64 = calories.iter().max().unwrap().clone();

    calories.sort();
    calories.reverse();
    let part2: u64 = calories[..3].iter().sum();

    (part1.to_string(), part2.to_string())
}
