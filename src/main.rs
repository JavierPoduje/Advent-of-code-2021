mod utils;

mod day_1;
mod day_2;
mod day_3;

mod parser;
use parser::Parser;

fn main() {
    let parser = Parser::new();

    let day = parser.args.value_of("day").unwrap();

    let (part1, part2) = match day {
        "1" => day_1::solution::solution(),
        "2" => day_2::solution::solution(),
        "3" => day_3::solution::solution(),
        _ => unreachable!(),
    };

    println!("- Day {}:", day);
    println!("\t* part 1: {}", part1);
    println!("\t* part 2: {}", part2);
}
