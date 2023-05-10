use std::collections::HashSet;

use nom::{character, sequence, IResult};

use crate::SolutionBase;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Movement = (Direction, u8);

#[derive(PartialEq, Debug)]
pub struct Solution {
    movements: Vec<Movement>,
}

fn parse_movement(input: &str) -> IResult<&str, Movement> {
    let (input, (direction, distance)) = sequence::separated_pair(
        character::complete::alpha1,
        character::complete::space1,
        character::complete::u8,
    )(input)?;
    let direction = match direction {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction {}", direction),
    };

    Ok((input, (direction, distance)))
}

#[derive(PartialEq, Eq, Hash, Clone, Default, Debug)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn move_one_step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn follow(&self, position: &Position) -> Option<Position> {
        let delta_x = position.x - self.x;
        let delta_y = position.y - self.y;

        if delta_x.abs() > 1 || delta_y.abs() > 1 {
            let x = self.x + delta_x.signum();
            let y = self.y + delta_y.signum();
            Some(Position { x, y })
        } else {
            None
        }
    }
}

fn get_tail_positions(size: usize, movements: &[Movement]) -> HashSet<Position> {
    assert!(size >= 1, "The rope should have at least one knot");

    let mut rope = vec![Position::default(); size];
    let mut tail_positions = HashSet::new();

    tail_positions.insert(rope.last().unwrap().clone());

    for (direction, distance) in movements {
        for _ in 0..*distance {
            rope[0].move_one_step(*direction);
            let mut all_moved = true;
            for i in 1..rope.len() {
                match rope[i].follow(&rope[i - 1]) {
                    Some(position) => rope[i] = position,
                    None => {
                        all_moved = false;
                        break;
                    }
                }
            }
            if all_moved {
                tail_positions.insert(rope.last().unwrap().clone());
            }
        }
    }

    tail_positions
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let movements = data
            .lines()
            .map(|line| parse_movement(line).unwrap().1)
            .collect();

        Solution {
            movements: movements,
        }
    }

    fn part_1(&self) -> String {
        let tail_positions = get_tail_positions(2, &self.movements);
        tail_positions.len().to_string()
    }

    fn part_2(&self) -> String {
        let tail_positions = get_tail_positions(10, &self.movements);
        tail_positions.len().to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution(example: &str) -> Solution {
        let data = fs::read_to_string(format!("data/day_09_{}.txt", example)).unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution("example_01");

        assert_eq!(
            solution,
            Solution {
                movements: vec![
                    (Direction::Right, 4),
                    (Direction::Up, 4),
                    (Direction::Left, 3),
                    (Direction::Down, 1),
                    (Direction::Right, 4),
                    (Direction::Down, 1),
                    (Direction::Left, 5),
                    (Direction::Right, 2),
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution("example_01");

        assert_eq!(solution.part_1(), "13");
    }

    #[test]
    fn part_2() {
        let solution = get_solution("example_02");

        assert_eq!(solution.part_2(), "36");
    }
}
