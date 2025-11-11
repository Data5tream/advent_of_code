use clap::{Command, arg, builder::styling, value_parser};

mod day_1;
mod day_2;
mod day_3;

pub fn run() -> Result<(), ()> {
    let matches = construct_command().get_matches();

    let day: Option<&u16> = matches.get_one("day");

    match day {
        None => unimplemented!(),
        Some(1) => day_1::run(),
        Some(2) => day_2::run(),
        Some(3) => day_3::run(),
        Some(day) => {
            eprintln!("Puzzle {day} is not solved yet");
            Err(())
        }
    }
}

fn construct_command() -> Command {
    Command::new("aoc2024")
        .about("Advent of Code 2024 | Implemented in rust")
        .after_help("Github repo: https://github.com/Data5tream/advent_of_code | Advent of Code 2024: https://adventofcode.com/2024/")
        .arg_required_else_help(true)
        .styles(build_clap_styles())
        .arg(
            arg!([day] "The day you want to run (1-25)")
                .required(true)
                .value_parser(value_parser!(u16).range(1..25)),
        )
}

fn build_clap_styles() -> styling::Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}
