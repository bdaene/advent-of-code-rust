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
        01 => Box::new(days::day_01::Solution::new(data)),
        02 => Box::new(days::day_02::Solution::new(data)),
        03 => Box::new(days::day_03::Solution::new(data)),
        04 => Box::new(days::day_04::Solution::new(data)),
        05 => Box::new(days::day_05::Solution::new(data)),
        06 => Box::new(days::day_06::Solution::new(data)),
        07 => Box::new(days::day_07::Solution::new(data)),
        08 => Box::new(days::day_08::Solution::new(data)),
        09 => Box::new(days::day_09::Solution::new(data)),
        10 => Box::new(days::day_10::Solution::new(data)),

        101 => Box::new(days::bonus_01::Solution::new(data)),
        _ => panic!("Invalid day"),
    }
}
