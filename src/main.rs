mod utils;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

mod parser;
use parser::Parser;

fn main() {
    let parser = Parser::new();

    let day = parser.args.value_of("day").unwrap();

    let (part1, part2) = match day {
        "1" => day_1::solution::solution(),
        "2" => day_2::solution::solution(),
        "3" => day_3::solution::solution(),
        "4" => day_4::solution::solution(),
        "5" => day_5::solution::solution(),
        "6" => day_6::solution::solution(),
        "7" => day_7::solution::solution(),
        "8" => day_8::solution::solution(),
        "9" => day_9::solution::solution(),
        "10" => day_10::solution::solution(),
        "11" => day_11::solution::solution(),
        "12" => day_12::solution::solution(),
        "13" => day_13::solution::solution(),
        _ => unreachable!(),
    };

    println!("- Day {}:", day);
    println!("\t* part 1: {}", part1);
    println!("\t* part 2: {}", part2);
}
