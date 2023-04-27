use itertools::Itertools;
use std::collections::HashSet;

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
pub struct Solution {
    rucksacks: Vec<String>,
}

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid char {c}"),
    }
}

fn get_common_character<'a>(sets: &Vec<&str>) -> char {
    sets.iter()
        .map(|set| {
            let set: HashSet<char> = HashSet::from_iter(set.chars().clone());
            set
        })
        .reduce(|a, b| &a & &b)
        .unwrap_or_default()
        .into_iter()
        .next()
        .expect("No common character")
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let rucksacks = data.lines().map(String::from).collect();

        Solution {
            rucksacks: rucksacks,
        }
    }

    fn part_1(&self) -> String {
        let mut total = 0;

        for rucksack in self.rucksacks.iter() {
            let compartements = rucksack.split_at(rucksack.len() / 2);
            total += get_priority(get_common_character(&vec![
                compartements.0,
                compartements.1,
            ]))
        }
        total.to_string()
    }

    fn part_2(&self) -> String {
        let mut total = 0;

        for elves in self.rucksacks.chunks(3) {
            let elves = elves.iter().map(String::as_str).collect_vec();
            total += get_priority(get_common_character(&elves))
        }
        total.to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data = fs::read_to_string("data/day_03_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                rucksacks: vec![
                    "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect(),
                    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".chars().collect(),
                    "PmmdzqPrVvPwwTWBwg".chars().collect(),
                    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".chars().collect(),
                    "ttgJtRGJQctTZtZT".chars().collect(),
                    "CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect(),
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "157");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "70");
    }
}
