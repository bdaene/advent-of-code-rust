use itertools::Itertools;
use nom::{branch, bytes, character, combinator, multi, sequence, IResult};

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}
#[derive(PartialEq, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

#[derive(PartialEq, Debug)]
pub struct Solution {
    monkeys: Vec<Monkey>,
}

fn parse_index(input: &str) -> IResult<&str, usize> {
    let (input, (_, index, _, _)) = sequence::tuple((
        bytes::complete::tag("Monkey "),
        character::complete::u8,
        bytes::complete::tag(":"),
        character::complete::line_ending,
    ))(input)?;

    Ok((input, index as usize))
}

fn parse_items(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, (_, _, items, _)) = sequence::tuple((
        character::complete::multispace1,
        bytes::complete::tag("Starting items: "),
        multi::separated_list0(bytes::complete::tag(", "), character::complete::u64),
        character::complete::line_ending,
    ))(input)?;

    Ok((input, items))
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (_, _, operation, _)) = sequence::tuple((
        character::complete::multispace1,
        bytes::complete::tag("Operation: new = "),
        sequence::tuple((
            character::complete::alphanumeric0,
            branch::alt((bytes::complete::tag(" + "), bytes::complete::tag(" * "))),
            character::complete::alphanumeric0,
        )),
        character::complete::line_ending,
    ))(input)?;

    let operation = match operation {
        ("old", " + ", value) => Operation::Add(value.parse().unwrap()),
        ("old", " * ", "old") => Operation::Square,
        ("old", " * ", value) => Operation::Mul(value.parse().unwrap()),
        _ => panic!("Invalid operation {:?}.", operation),
    };

    Ok((input, operation))
}

fn parse_test(input: &str) -> IResult<&str, u64> {
    let (input, (_, _, test, _)) = sequence::tuple((
        character::complete::multispace1,
        bytes::complete::tag("Test: divisible by "),
        character::complete::u64,
        character::complete::line_ending,
    ))(input)?;

    Ok((input, test))
}

fn parse_throw(input: &str) -> IResult<&str, usize> {
    let (input, (_, _, _, _, target, _)) = sequence::tuple((
        character::complete::multispace1,
        bytes::complete::tag("If "),
        branch::alt((bytes::complete::tag("true"), bytes::complete::tag("false"))),
        bytes::complete::tag(": throw to monkey "),
        character::complete::u8,
        character::complete::line_ending,
    ))(input)?;

    let target = target as usize;

    Ok((input, target))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, (_, items, operation, test, monkey_if_true, monkey_if_false, _)) =
        sequence::tuple((
            parse_index,
            parse_items,
            parse_operation,
            parse_test,
            parse_throw,
            parse_throw,
            combinator::opt(character::complete::line_ending),
        ))(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            monkey_if_true,
            monkey_if_false,
        },
    ))
}

#[derive(Debug)]
struct MonkeyState<'a> {
    monkey: &'a Monkey,
    items: Vec<u64>,
    inspections: u64,
}

impl<'a> MonkeyState<'a> {
    fn new(monkey: &'a Monkey) -> Self {
        Self {
            monkey,
            items: monkey.items.clone(),
            inspections: 0,
        }
    }
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let (_, monkeys) = multi::many0(parse_monkey)(data).unwrap();
        Solution { monkeys }
    }

    fn part_1(&self) -> String {
        let mut monkeys: Vec<MonkeyState> = self
            .monkeys
            .iter()
            .map(|monkey| MonkeyState::new(monkey))
            .collect();

        for _ in 1..=20 {
            for i in 0..monkeys.len() {
                for item in monkeys[i].items.clone() {
                    let item = match monkeys[i].monkey.operation {
                        Operation::Add(value) => item + value,
                        Operation::Mul(value) => item * value,
                        Operation::Square => item * item,
                    } / 3;
                    let target = match item % monkeys[i].monkey.test {
                        0 => monkeys[i].monkey.monkey_if_true,
                        _ => monkeys[i].monkey.monkey_if_false,
                    };
                    monkeys[target].items.push(item)
                }
                monkeys[i].inspections += monkeys[i].items.len() as u64;
                monkeys[i].items.clear();
            }
        }

        monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .sorted()
            .rev()
            .take(2)
            .fold(1, |acc, value| acc * value)
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut monkeys: Vec<MonkeyState> = self
            .monkeys
            .iter()
            .map(|monkey| MonkeyState::new(monkey))
            .collect();

        let modulus = self.monkeys.iter().fold(1, |acc, monkey| acc * monkey.test);

        for _ in 1..=10_000 {
            for i in 0..monkeys.len() {
                for item in monkeys[i].items.clone() {
                    let item = match monkeys[i].monkey.operation {
                        Operation::Add(value) => item + value,
                        Operation::Mul(value) => item * value,
                        Operation::Square => item * item,
                    } % modulus;
                    let target = match item % monkeys[i].monkey.test {
                        0 => monkeys[i].monkey.monkey_if_true,
                        _ => monkeys[i].monkey.monkey_if_false,
                    };
                    monkeys[target].items.push(item)
                }
                monkeys[i].inspections += monkeys[i].items.len() as u64;
                monkeys[i].items.clear();
            }
        }

        monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .sorted()
            .rev()
            .take(2)
            .fold(1, |acc, value| acc * value as u64)
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data = fs::read_to_string("data/day_11_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                monkeys: vec![
                    Monkey {
                        items: vec![79, 98],
                        operation: Operation::Mul(19),
                        test: 23,
                        monkey_if_true: 2,
                        monkey_if_false: 3,
                    },
                    Monkey {
                        items: vec![54, 65, 75, 74],
                        operation: Operation::Add(6),
                        test: 19,
                        monkey_if_true: 2,
                        monkey_if_false: 0,
                    },
                    Monkey {
                        items: vec![79, 60, 97],
                        operation: Operation::Square,
                        test: 13,
                        monkey_if_true: 1,
                        monkey_if_false: 3,
                    },
                    Monkey {
                        items: vec![74],
                        operation: Operation::Add(3),
                        test: 17,
                        monkey_if_true: 0,
                        monkey_if_false: 1,
                    },
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "10605");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "2713310158");
    }
}
