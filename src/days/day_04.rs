use nom::{bytes, character, sequence, IResult};

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
struct Assignement(u8, u8);

#[derive(PartialEq, Debug)]
pub struct Solution {
    pairs: Vec<(Assignement, Assignement)>,
}

fn parse_assignement(input: &str) -> IResult<&str, Assignement> {
    let (input, (a, b)) = sequence::separated_pair(
        character::complete::u8,
        bytes::complete::tag("-"),
        character::complete::u8,
    )(input)?;
    Ok((input, Assignement(a, b)))
}

fn parse_line(input: &str) -> IResult<&str, (Assignement, Assignement)> {
    let (input, (a, b)) = sequence::separated_pair(
        parse_assignement,
        bytes::complete::tag(","),
        parse_assignement,
    )(input)?;
    Ok((input, (a, b)))
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let pairs = data
            .lines()
            .map(|line| parse_line(line).unwrap().1)
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
