#![allow(unused_variables)]
mod days;

use days::{
    Day, day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12,
};
use std::env;

fn main() {
    let cargo_root = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set")
        .to_string()
        + "/inputs/";

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Enter a day to run");
        std::process::exit(1);
    }

    let day: u32 = match args[1].parse() {
        Ok(n) if n <= 25 => n,
        _ => {
            eprintln!("Please provide a number between 0 and 25.");
            std::process::exit(1);
        }
    };

    run_day(day, cargo_root)
}

fn run_day(day: u32, root_directory: String) {
    match day {
        1 => day01::Day01::run(root_directory),
        2 => day02::Day02::run(root_directory),
        3 => day03::Day03::run(root_directory),
        4 => day04::Day04::run(root_directory),
        5 => day05::Day05::run(root_directory),
        6 => day06::Day06::run(root_directory),
        7 => day07::Day07::run(root_directory),
        8 => day08::Day08::run(root_directory),
        9 => day09::Day09::run(root_directory),
        10 => day10::Day10::run(root_directory),
        11 => day11::Day11::run(root_directory),
        12 => day12::Day12::run(root_directory),
        d => panic!("Provided unsupported day {}", d),
    }
}
