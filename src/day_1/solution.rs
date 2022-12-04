use super::super::utils::read_one_per_line::read_one_per_line;

pub fn solution() -> (u64, u64) {
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

    let max: u64 = calories.iter().max().unwrap().clone();

    (max, 0)
}
