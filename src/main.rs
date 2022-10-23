mod utils;

mod day_1;
mod day_2;
mod day_3;

fn main() {
    let (part1, part2) = day_1::solution::solution();
    println!("- Day 1");
    println!("\t* part 1: {}", part1);
    println!("\t* part 2: {}", part2);

    let (part1, part2) = day_2::solution::solution();
    println!("- Day 2");
    println!("\t* part 1: {}", part1);
    println!("\t* part 2: {}", part2);

    let (part1, part2) = day_3::solution::solution();
    println!("- Day 3");
    println!("\t* part 1: {}", part1);
    println!("\t* part 2: {}", part2);
}
