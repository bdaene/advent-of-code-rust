pub mod days;

pub trait Solution {
    fn new(data: &str) -> Self;

    fn part_1(&self) -> String {
        String::from("Not implemented yet.")
    }

    fn part_2(&self) -> String{
        String::from("Not implemented yet.")
    }
}

pub fn get_solution(day: u8, data: &str) -> days::day_01::Day01Solution {
    match day {
        1 => days::day_01::Day01Solution::new(data),
        _ => panic!("Invalid day"),
    }
}