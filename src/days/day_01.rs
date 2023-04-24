use itertools::Itertools;

use crate::Solution;

type Elf= Vec<u32>;

#[derive(Debug)]
pub struct Day01Solution{
    elves: Vec<Elf>,
}

impl Day01Solution {
    fn parse_data(input: &str) -> Vec<Elf> {
        input.split("\n\n").map(
            |elf| elf.lines().filter_map(
                |calories| calories.parse().ok()
            ).collect()
        ).collect()
    }
}

impl Solution for Day01Solution {
    fn new(input: &str) -> Self {
        Day01Solution {
            elves: Day01Solution::parse_data(input)
        }
    }

    fn part_1(&self) -> String {
        self.elves.iter().map(
            |elf| elf.iter().sum::<u32>()
        ).max().unwrap().to_string()
    }

    fn part_2(&self) -> String {
        self.elves.iter().map(
            |elf| elf.iter().sum::<u32>()
        ).sorted().rev().take(3).sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::Solution;

    use super::Day01Solution;

    fn get_solution() -> Day01Solution {
        let data:String = fs::read_to_string("data/day_01_example.txt").unwrap();

        Day01Solution::new(&data)
    }

    #[test]
    fn new () {
        get_solution();
    }

    #[test]
    fn part_1 () {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "24000");
    }

    #[test]
    fn part_2 () {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "45000");
    }
}