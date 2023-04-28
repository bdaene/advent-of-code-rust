use itertools::Itertools;

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
struct Assignement(u8, u8);

#[derive(PartialEq, Debug)]
pub struct Solution {
    pairs: Vec<(Assignement, Assignement)>,
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let pairs = data
            .lines()
            .map(|assignements| {
                assignements
                    .split(',')
                    .map(|assignement| {
                        let (start, end) = assignement
                            .split('-')
                            .map(|bound| bound.parse().expect("Should be u8."))
                            .collect_tuple()
                            .expect("Could not parse assignement.");
                        Assignement(start, end)
                    })
                    .collect_tuple()
                    .expect("Could not parse assignements")
            })
            .collect();

        Solution { pairs }
    }

    fn part_1(&self) -> String {
        self.pairs
            .iter()
            .map(|(a, b)| ((a.0 <= b.0 && b.1 <= a.1) || (b.0 <= a.0 && a.1 <= b.1)) as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.pairs
            .iter()
            .map(|(a, b)| (a.0 <= b.1 && b.0 <= a.1) as u32)
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
        let data = fs::read_to_string("data/day_04_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                pairs: vec![
                    (Assignement(2, 4), Assignement(6, 8)),
                    (Assignement(2, 3), Assignement(4, 5)),
                    (Assignement(5, 7), Assignement(7, 9)),
                    (Assignement(2, 8), Assignement(3, 7)),
                    (Assignement(6, 6), Assignement(4, 6)),
                    (Assignement(2, 6), Assignement(4, 8)),
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "2");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "4");
    }
}
