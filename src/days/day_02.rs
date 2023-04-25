use crate::Solution;

#[derive(Debug)]
pub struct Day02Solution{
    strategy_guide: Vec<(char, char)>,
}



impl Solution for Day02Solution {
    fn new(input: &str) -> Self {
        let mut strategy = Vec::new();

        for line in input.lines() {
            let mut rule = line.chars();

            let opponent = rule.next().expect("Missing opponent part of rule.");
            assert!(rule.next().unwrap().is_whitespace());
            let me = rule.next().expect("Missing me part of rule.");
            
            strategy.push((opponent as char, me as char));
        }

        Day02Solution {
            strategy_guide: strategy,
        }
    }

    fn part_1(&self) -> String {
        self.strategy_guide.iter().map(
            |rule| match rule {
                ('A', 'X') => 1 + 3,
                ('B', 'X') => 1 + 0,
                ('C', 'X') => 1 + 6,
                ('A', 'Y') => 2 + 6,
                ('B', 'Y') => 2 + 3,
                ('C', 'Y') => 2 + 0,
                ('A', 'Z') => 3 + 0,
                ('B', 'Z') => 3 + 6,
                ('C', 'Z') => 3 + 3,
                _ => panic!("Invalid rule.")
            } ).sum::<u32>().to_string()
    }

    fn part_2(&self) -> String {
        self.strategy_guide.iter().map(
            |rule| match rule {
                ('A', 'X') => 3 + 0,
                ('B', 'X') => 1 + 0,
                ('C', 'X') => 2 + 0,
                ('A', 'Y') => 1 + 3,
                ('B', 'Y') => 2 + 3,
                ('C', 'Y') => 3 + 3,
                ('A', 'Z') => 2 + 6,
                ('B', 'Z') => 3 + 6,
                ('C', 'Z') => 1 + 6,
                _ => panic!("Invalid rule.")
            } ).sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::Solution;

    use super::Day02Solution;

    fn get_solution() -> Day02Solution {
        let data:String = fs::read_to_string("data/day_02_example.txt").unwrap();

        Day02Solution::new(&data)
    }

    #[test]
    fn new () {
        get_solution();
    }

    #[test]
    fn part_1 () {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "15");
    }

    #[test]
    fn part_2 () {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "12");
    }
}