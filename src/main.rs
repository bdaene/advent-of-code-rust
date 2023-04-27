use std::fs;
use std::time;

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

    let start = time::Instant::now();
    let solution = get_solution(args.day, &data);
    println!("Parsed data in {:?}", start.elapsed());

    let start = time::Instant::now();
    let answer = solution.part_1();
    println!("Part 1 ({:?}): {}", start.elapsed(), answer);

    let start = time::Instant::now();
    let answer = solution.part_2();
    println!("Part 2 ({:?}): {}", start.elapsed(), answer);
}
