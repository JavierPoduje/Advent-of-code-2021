mod utils;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let (_part1, _part2) = day_1::solution::solution();
    println!("- Day 1");
    println!("\t* part 1: {}", _part1);
    println!("\t* part 2: {}", _part2);

    let (_part1, _part2) = day_2::solution::solution();
    println!("- Day 2");
    println!("\t* part 1: {}", _part1);
    println!("\t* part 2: {}", _part2);

    let (_part1, _part2) = day_3::solution::solution();
    println!("- Day 3");
    println!("\t* part 1: {}", _part1);
    println!("\t* part 2: {}", _part2);

    let (part1, part2) = day_4::solution::solution();
    println!("- Day 4");
    println!("\t* part 1: {}", part1);
    println!("\t* part 2: {}", part2);
}
