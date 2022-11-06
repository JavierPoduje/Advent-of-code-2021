use clap::{Arg, ArgMatches, Command};

pub struct Parser {
    pub args: ArgMatches,
}

impl Parser {
    pub fn new() -> Self {
        let args = Command::new("Aoc 2021")
            .version("0.1")
            .about("Solutions to advent of code - 2021")
            .arg(
                Arg::new("day")
                    .long("day")
                    .help("Day to execute")
                    .takes_value(true)
                    .required(true),
            )
            .get_matches();
        Self { args }
    }
}
