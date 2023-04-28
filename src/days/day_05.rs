use nom::{bytes, character, sequence, IResult};

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
struct Move(usize, usize, usize);

#[derive(PartialEq, Debug)]
pub struct Solution {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn parse_stacks(input: &Vec<&str>) -> Vec<Vec<char>> {
    let input: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let n = input.len();
    let width = (input[n - 1].len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = (0..width).map(|_| Vec::new()).collect();

    for (i, stack) in stacks.iter_mut().enumerate() {
        let column = i * 4 + 1;
        for j in 2..=n {
            let c = input[n - j][column];
            if c.is_uppercase() {
                stack.push(c)
            }
        }
    }

    stacks
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    // move 1 from 2 to 1
    let (input, (_, quantity, _, from, _, to)) = sequence::tuple((
        bytes::complete::tag("move "),
        character::complete::u8,
        bytes::complete::tag(" from "),
        character::complete::u8,
        bytes::complete::tag(" to "),
        character::complete::u8,
    ))(input)?;
    Ok((
        input,
        Move(quantity as usize, from as usize - 1, to as usize - 1),
    ))
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let mut boat = Vec::new();
        let mut lines = data.lines();

        for line in lines.by_ref().take_while(|line| !line.is_empty()) {
            boat.push(line)
        }

        Solution {
            stacks: parse_stacks(&boat),
            moves: lines.map(|line| parse_move(line).unwrap().1).collect(),
        }
    }

    fn part_1(&self) -> String {
        let mut stacks: Vec<Vec<char>> = self.stacks.iter().map(|stack| stack.clone()).collect();

        for Move(quantity, from, to) in self.moves.iter() {
            for _ in 0..*quantity {
                let boat_crate = stacks[*from]
                    .pop()
                    .expect("Tried to move from an empty stack.");
                stacks[*to].push(boat_crate);
            }
        }

        stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }

    fn part_2(&self) -> String {
        let mut stacks: Vec<Vec<char>> = self.stacks.iter().map(|stack| stack.clone()).collect();

        for Move(quantity, from, to) in self.moves.iter() {
            let n = stacks[*from].len();
            let top_crates: Vec<_> = stacks[*from].drain(n - quantity..n).collect();
            stacks[*to].extend(top_crates);
        }

        stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data = fs::read_to_string("data/day_05_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P'],],
                moves: vec![Move(1, 1, 0), Move(3, 0, 2), Move(2, 1, 0), Move(1, 0, 1)],
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "CMZ");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "MCD");
    }
}
