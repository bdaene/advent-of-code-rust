use itertools::Itertools;

use crate::SolutionBase;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Command {
    Addx(i16),
    Noop,
}

#[derive(PartialEq, Debug)]
pub struct Solution {
    commands: Vec<Command>,
}

fn parse_command(input: &str) -> Command {
    match input {
        "noop" => Command::Noop,
        _ => Command::Addx(
            input
                .strip_prefix("addx ")
                .expect("Should be 'addx V'")
                .parse()
                .expect("V should be an integer"),
        ),
    }
}

struct RegisterIterator<'a> {
    commands: std::slice::Iter<'a, Command>,
    cycles: u8,
    x: i16,
    new_x: i16,
}

impl<'a> RegisterIterator<'a> {
    fn new(commands: &'a Vec<Command>) -> RegisterIterator<'a> {
        RegisterIterator {
            commands: commands.iter(),
            cycles: 0,
            x: 1,
            new_x: 1,
        }
    }
}

impl<'a> Iterator for RegisterIterator<'a> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cycles == 0 {
            self.x = self.new_x;
            match self.commands.next() {
                Some(Command::Addx(value)) => {
                    self.cycles = 2;
                    self.new_x += value
                }
                Some(Command::Noop) => self.cycles = 1,
                None => return None,
            }
        }
        self.cycles -= 1;
        Some(self.x)
    }
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let commands = data.lines().map(|line| parse_command(line)).collect();
        Solution { commands }
    }

    fn part_1(&self) -> String {
        let register_iterator = RegisterIterator::new(&self.commands);

        register_iterator
            .enumerate()
            .skip(19)
            .step_by(40)
            .map(|(i, x)| (i + 1) as i16 * x)
            .sum::<i16>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let register_iterator = RegisterIterator::new(&self.commands);

        let ouput =        register_iterator
            .enumerate()
            .map(|(i, x)| {
                if ((i as i16) % 40 - x).abs() <= 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .chunks(40)
            .into_iter()
            .map(|pixels| pixels.into_iter().join(""))
            .join("\n");
        "\n".to_string() + ouput.as_str()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data = fs::read_to_string("data/day_10_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                commands: vec![
                    Command::Addx(15),
                    Command::Addx(-11),
                    Command::Addx(6),
                    Command::Addx(-3),
                    Command::Addx(5),
                    Command::Addx(-1),
                    Command::Addx(-8),
                    Command::Addx(13),
                    Command::Addx(4),
                    Command::Noop,
                    Command::Addx(-1),
                    Command::Addx(5),
                    Command::Addx(-1),
                    Command::Addx(5),
                    Command::Addx(-1),
                    Command::Addx(5),
                    Command::Addx(-1),
                    Command::Addx(5),
                    Command::Addx(-1),
                    Command::Addx(-35),
                    Command::Addx(1),
                    Command::Addx(24),
                    Command::Addx(-19),
                    Command::Addx(1),
                    Command::Addx(16),
                    Command::Addx(-11),
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(21),
                    Command::Addx(-15),
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(-3),
                    Command::Addx(9),
                    Command::Addx(1),
                    Command::Addx(-3),
                    Command::Addx(8),
                    Command::Addx(1),
                    Command::Addx(5),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(-36),
                    Command::Noop,
                    Command::Addx(1),
                    Command::Addx(7),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(2),
                    Command::Addx(6),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(1),
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(7),
                    Command::Addx(1),
                    Command::Noop,
                    Command::Addx(-13),
                    Command::Addx(13),
                    Command::Addx(7),
                    Command::Noop,
                    Command::Addx(1),
                    Command::Addx(-33),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(2),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(8),
                    Command::Noop,
                    Command::Addx(-1),
                    Command::Addx(2),
                    Command::Addx(1),
                    Command::Noop,
                    Command::Addx(17),
                    Command::Addx(-9),
                    Command::Addx(1),
                    Command::Addx(1),
                    Command::Addx(-3),
                    Command::Addx(11),
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(1),
                    Command::Noop,
                    Command::Addx(1),
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(-13),
                    Command::Addx(-19),
                    Command::Addx(1),
                    Command::Addx(3),
                    Command::Addx(26),
                    Command::Addx(-30),
                    Command::Addx(12),
                    Command::Addx(-1),
                    Command::Addx(3),
                    Command::Addx(1),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(-9),
                    Command::Addx(18),
                    Command::Addx(1),
                    Command::Addx(2),
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(9),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(-1),
                    Command::Addx(2),
                    Command::Addx(-37),
                    Command::Addx(1),
                    Command::Addx(3),
                    Command::Noop,
                    Command::Addx(15),
                    Command::Addx(-21),
                    Command::Addx(22),
                    Command::Addx(-6),
                    Command::Addx(1),
                    Command::Noop,
                    Command::Addx(2),
                    Command::Addx(1),
                    Command::Noop,
                    Command::Addx(-10),
                    Command::Noop,
                    Command::Noop,
                    Command::Addx(20),
                    Command::Addx(1),
                    Command::Addx(2),
                    Command::Addx(2),
                    Command::Addx(-6),
                    Command::Addx(-11),
                    Command::Noop,
                    Command::Noop,
                    Command::Noop
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "13140");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(
            solution.part_2(),
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
    }
}
