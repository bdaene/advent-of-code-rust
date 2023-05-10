use std::collections::HashSet;

use nom::{character, sequence, IResult};

use crate::SolutionBase;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    U,
    D,
    L,
    R,
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
    let movement = match direction {
        "U" => (Direction::U, distance),
        "D" => (Direction::D, distance),
        "L" => (Direction::L, distance),
        "R" => (Direction::R, distance),
        _ => panic!("Unknown direction {}", direction),
    };

    Ok((input, movement))
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn update(&mut self, direction: Direction, distance: u8) {
        let distance = distance as i16;

        match direction {
            Direction::U => self.y += distance,
            Direction::D => self.y -= distance,
            Direction::L => self.x -= distance,
            Direction::R => self.x += distance,
        }
    }

    fn follow(&mut self, position: &Position) {
        let distance = (position.x - self.x).abs().max((position.y - self.y).abs());
        if distance > 1 {
            if self.x < position.x {
                self.x += 1
            }
            if self.x > position.x {
                self.x -= 1
            }
            if self.y < position.y {
                self.y += 1
            }
            if self.y > position.y {
                self.y -= 1
            }
        }
    }
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
        let mut head = Position { x: 0, y: 0 };
        let mut tail = Position { x: 0, y: 0 };

        let mut visited = HashSet::new();
        visited.insert(tail.clone());

        for (direction, distance) in self.movements.iter() {
            for _ in 0..*distance {
                head.update(*direction, 1);
                tail.follow(&head);
                visited.insert(tail.clone());
            }
        }

        visited.len().to_string()
    }

    fn part_2(&self) -> String {
        let mut rope = vec![Position{x:0, y:0}; 10];
        let mut visited = HashSet::new();

        visited.insert(rope[9].clone());

        for (direction, distance) in self.movements.iter() {
            for _ in 0..*distance {
                rope[0].update(*direction, 1);
                for i in 1..rope.len() {
                    let (left, right) = rope.split_at_mut(i);
                    right[0].follow(&left[i-1]);
                }
                visited.insert(rope[9].clone());
            }
        }

        visited.len().to_string()
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
                    (Direction::R, 4),
                    (Direction::U, 4),
                    (Direction::L, 3),
                    (Direction::D, 1),
                    (Direction::R, 4),
                    (Direction::D, 1),
                    (Direction::L, 5),
                    (Direction::R, 2),
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
