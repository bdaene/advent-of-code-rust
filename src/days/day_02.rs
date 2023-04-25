use crate::Solution;

#[derive(Debug)]
pub struct Day02Solution {
    strategy_guide: Vec<(u8, u8)>,
}

impl Solution for Day02Solution {
    fn new(input: &str) -> Self {
        let mut strategy = Vec::new();

        for line in input.lines() {
            let mut rule = line.chars();

            let opponent = rule.next().expect("Missing opponent part of rule.");
            assert!(rule.next().expect("Missing whitespace part in rule.").is_whitespace());
            let me = rule.next().expect("Missing me part of rule.");

            strategy.push((opponent as u8 - 'A' as u8, me as u8 - 'X' as u8));
        }

        Day02Solution {
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
    use crate::Solution;
    use std::fs;

    use super::Day02Solution;

    fn get_solution() -> Day02Solution {
        let data = fs::read_to_string("data/day_02_example.txt").unwrap();

        Day02Solution::new(&data)
    }

    #[test]
    fn new() {
        get_solution();
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
