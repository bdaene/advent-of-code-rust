use itertools::Itertools;

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
struct Elf {
    calories: Vec<u32>,
}

impl Elf {
    fn get_total_calories(&self) -> u32 {
        self.calories.iter().sum::<u32>()
    }
}

#[derive(PartialEq, Debug)]
pub struct Solution {
    elves: Vec<Elf>,
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let elves = data
            .split("\n\n")
            .map(|elf| {
                let calories = elf
                    .lines()
                    .filter_map(|calories| calories.parse().ok())
                    .collect();
                Elf { calories: calories }
            })
            .collect();

        Solution { elves: elves }
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
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data: String = fs::read_to_string("data/day_01_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                elves: vec![
                    Elf {
                        calories: vec![1000, 2000, 3000]
                    },
                    Elf {
                        calories: vec![4000]
                    },
                    Elf {
                        calories: vec![5000, 6000]
                    },
                    Elf {
                        calories: vec![7000, 8000, 9000]
                    },
                    Elf {
                        calories: vec![10000]
                    },
                ]
            }
        )
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
