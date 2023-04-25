use itertools::Itertools;

use crate::Solution;

struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn get_total_calories(&self) -> u32 {
        self.calories.iter().sum::<u32>()
    }
}

pub struct Day01Solution {
    elves: Vec<Elf>,
}

impl Solution for Day01Solution {
    fn new(input: &str) -> Self {
        let elves = input
            .split("\n\n")
            .map(|elf| {
                let calories = elf
                    .lines()
                    .filter_map(|calories| calories.parse().ok())
                    .collect();
                Elf { calories: calories }
            })
            .collect();

        Day01Solution { elves: elves }
    }

    fn part_1(&self) -> String {
        self.elves
            .iter()
            .map(|elf| elf.get_total_calories())
            .max()
            .unwrap()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.elves
            .iter()
            .map(|elf| elf.get_total_calories())
            .sorted()
            .rev()
            .take(3)
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;
    use std::fs;

    use super::Day01Solution;

    fn get_solution() -> Day01Solution {
        let data: String = fs::read_to_string("data/day_01_example.txt").unwrap();

        Day01Solution::new(&data)
    }

    #[test]
    fn new() {
        get_solution();
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "24000");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "45000");
    }
}
