use std::collections::HashMap;

use itertools::{izip, Itertools};

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
pub struct Solution {
    buffers: Vec<String>,
}

fn get_first_marker_position(buffer: &str, marker_length: usize) -> Option<usize> {
    let mut letters: HashMap<char, usize> = HashMap::new();
    for c in buffer.chars().take(marker_length) {
        letters.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    let mut count = letters.len();
    if count == marker_length {
        return Some(marker_length);
    }

    for (i, (a, b)) in izip!(buffer.chars(), buffer.chars().skip(marker_length)).enumerate() {
        if *(letters.entry(a).and_modify(|v| *v -= 1).or_insert(0)) == 0 {
            count -= 1;
        }
        if *(letters.entry(b).and_modify(|v| *v += 1).or_insert(1)) == 1 {
            count += 1;
        }
        if count == marker_length {
            return Some(i + marker_length + 1);
        }
    }

    None
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let buffers = data.lines().map(String::from).collect();

        Solution { buffers }
    }

    fn part_1(&self) -> String {
        self.buffers
            .iter()
            .map(|buffer| get_first_marker_position(buffer, 4))
            .map(|answer| answer.unwrap_or(0).to_string())
            .collect_vec()
            .join(",")
    }

    fn part_2(&self) -> String {
        self.buffers
            .iter()
            .map(|buffer| get_first_marker_position(buffer, 14))
            .map(|answer| answer.unwrap_or(0).to_string())
            .collect_vec()
            .join(",")
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data = fs::read_to_string("data/day_06_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                buffers: vec![
                    "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),
                    "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
                    "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
                    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
                    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),
                ]
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "7,5,6,10,11");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "19,23,23,29,26");
    }
}
