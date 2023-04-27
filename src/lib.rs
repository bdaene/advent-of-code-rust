pub mod days;

pub trait SolutionBase {
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

pub fn get_solution(day: u8, data: &str) -> Box<dyn SolutionBase> {
    match day {
        1 => Box::new(days::day_01::Solution::new(data)),
        2 => Box::new(days::day_02::Solution::new(data)),
        101 => Box::new(days::bonus_01::Solution::new(data)),
        _ => panic!("Invalid day"),
    }
}
