use aoc2025::{
    day01::Day01, day02::Day02, day03::Day03, day04::Day04, day05::Day05, solution::solve,
};
use clap::Parser;
use colored::Colorize;
use std::fs;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    day: u8,
}

fn load_input(day: u8) -> String {
    let path = format!("./input/day{:02}.txt", day);
    fs::read_to_string(path).expect("input file should read")
}

fn run_solution(day: u8, input: &str) -> (String, String) {
    match day {
        1 => solve::<Day01>(input),
        2 => solve::<Day02>(input),
        3 => solve::<Day03>(input),
        4 => solve::<Day04>(input),
        5 => solve::<Day05>(input),
        _ => panic!("invalid day: {}", day),
    }
}

fn main() {
    let args = Args::parse();
    let input = load_input(args.day);
    let (output1, output2) = run_solution(args.day, &input);
    println!(
        "{}{}\n{}{}\n{}{}",
        "Advent of Code 2025, day ".bold().underline(),
        args.day.to_string().bold().underline(),
        "Part 1: ".bold(),
        output1,
        "Part 2: ".bold(),
        output2,
    );
}
