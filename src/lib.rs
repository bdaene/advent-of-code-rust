pub mod days;

pub trait Solution {
    fn new(data: &str) -> Self
    where
        Self: Sized;

    fn part_1(&self) -> String {
        String::from("Not implemented yet.")
    }

    fn part_2(&self) -> String {
        String::from("Not implemented yet.")
    }
}

pub fn get_solution(day: u8, data: &str) -> Box<dyn Solution> {
    match day {
        1 => Box::new(days::day_01::Day01Solution::new(data)),
        2 => Box::new(days::day_02::Day02Solution::new(data)),
        _ => panic!("Invalid day"),
    }
}
