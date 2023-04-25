use std::fs;

use advent_of_code_2022::get_solution;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Puzzle day
    day: u8,
    /// data path
    data: String,
}

fn main() {
    let args = Args::parse();

    let data = fs::read_to_string(args.data).expect("Input data not found.");

    let solution = get_solution(args.day, &data);

    println!("Part 1: {}", solution.part_1());
    println!("Part 2: {}", solution.part_2());
}
