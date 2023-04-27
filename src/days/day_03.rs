use itertools::Itertools;
use std::collections::HashSet;

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
pub struct Solution {
    rucksacks: Vec<Vec<char>>,
}

fn get_priority(c: char) -> u32 {
    let offset = match c {
        'a'..='z' => 'a' as i8 - 1,
        'A'..='Z' => 'A' as i8 - 27,
        _ => panic!("Invalid char {c}"),
    };
    (c as i8 - offset) as u32
}

fn char_set_unique_intersection<I, Set>(sets: I) -> char
where
    I: IntoIterator<Item = Set>,
    Set: IntoIterator<Item = char>,
{
    sets.into_iter()
        .map(|set| {
            let set: HashSet<char> = HashSet::from_iter(set);
            set
        })
        .reduce(|l, r| &l & &r)
        .unwrap_or_default()
        .into_iter()
        .next()
        .expect("No common element")
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let rucksacks = data.lines().map(|line| line.chars().collect()).collect();

        Solution {
            rucksacks: rucksacks,
        }
    }

    fn part_1(&self) -> String {
        self.rucksacks
            .iter()
            .map(|rucksack| {
                let left = rucksack[..rucksack.len()/2].iter().cloned();
                let right = rucksack[rucksack.len()/2..].iter().cloned();
                let compartements = vec![left, right];
                get_priority(char_set_unique_intersection(compartements))
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.rucksacks
            .iter()
            .chunks(3)
            .into_iter()
            .map(|elves| get_priority(char_set_unique_intersection(elves.cloned())))
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
