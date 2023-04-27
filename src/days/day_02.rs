use crate::SolutionBase;

#[derive(PartialEq, Debug)]
pub struct Solution {
    strategy_guide: Vec<(u8, u8)>,
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let mut strategy = Vec::new();

        for line in data.lines() {
            let mut rule = line.chars();

            let opponent = rule.next().expect("Missing opponent part of rule.");
            assert!(rule
                .next()
                .expect("Missing whitespace part in rule.")
                .is_whitespace());
            let me = rule.next().expect("Missing me part of rule.");

            strategy.push((opponent as u8 - 'A' as u8, me as u8 - 'X' as u8));
        }

        Solution {
            strategy_guide: strategy,
        }
    }

    fn part_1(&self) -> String {
        self.strategy_guide
            .iter()
            .map(|(opponent, me)| ((1 + me) + (4 + me - opponent) % 3 * 3) as u32)
            .sum::<u32>()
            .to_string()
    }

    fn part_2(&self) -> String {
        self.strategy_guide
            .iter()
            .map(|(opponent, rule)| ((opponent + rule + 2) % 3 + 1 + rule * 3) as u32)
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
        let data = fs::read_to_string("data/day_02_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                strategy_guide: vec![(0, 1), (1, 0), (2, 2)]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "15");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "12");
    }
}
