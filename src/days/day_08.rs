use itertools::{izip, Itertools};

use crate::SolutionBase;

#[derive(PartialEq, Debug)]
pub struct Solution {
    width: usize,
    height: usize,
    trees: Vec<u8>,
}

fn mark_visible_trees(
    trees: &Vec<u8>,
    positions: &mut impl Iterator<Item = usize>,
    visible: &mut Vec<bool>,
) {
    let position = positions.next().expect("At least one position to check.");
    let mut max_heigth = trees[position];
    visible[position] = true;

    for position in positions {
        if trees[position] > max_heigth {
            visible[position] = true;
            max_heigth = trees[position];
        }
    }
}

fn compute_visible_distance(
    trees: &Vec<u8>,
    positions: &mut impl Iterator<Item = usize>,
    distance: &mut Vec<usize>,
) {
    let mut distances: Vec<(u8, usize)> = vec![];
    for (i, position) in positions.enumerate() {
        while distances
            .last()
            .map_or(false, |(h, _)| *h < trees[position])
        {
            distances.pop();
        }
        distance[position] = i - distances.last().map_or(0, |(_, j)| *j);
        if distances
            .last()
            .map_or(false, |(h, _)| *h == trees[position])
        {
            distances.pop();
        }
        distances.push((trees[position], i));
    }
}

impl SolutionBase for Solution {
    fn new(data: &str) -> Self {
        let trees: Vec<Vec<u8>> = data
            .lines()
            .map(|line| line.chars().map(|tree| tree as u8 - '0' as u8).collect())
            .collect();

        Solution {
            height: trees.len(),
            width: trees[0].len(),
            trees: trees.into_iter().flatten().collect_vec(),
        }
    }

    fn part_1(&self) -> String {
        let mut visible = vec![false; self.trees.len()];

        for position in 0..self.width {
            let column = (position..self.trees.len()).step_by(self.width);

            mark_visible_trees(&self.trees, &mut column.clone(), &mut visible);
            mark_visible_trees(&self.trees, &mut column.clone().rev(), &mut visible);
        }

        for position in 0..self.height {
            let row = position * self.width..(position + 1) * self.width;

            mark_visible_trees(&self.trees, &mut row.clone(), &mut visible);
            mark_visible_trees(&self.trees, &mut row.clone().rev(), &mut visible);
        }

        visible
            .iter()
            .map(|b| *b as usize)
            .sum::<usize>()
            .to_string()
    }

    fn part_2(&self) -> String {
        let mut distance_up = vec![0_usize; self.trees.len()];
        let mut distance_down = vec![0_usize; self.trees.len()];
        let mut distance_left = vec![0_usize; self.trees.len()];
        let mut distance_right = vec![0_usize; self.trees.len()];

        for position in 0..self.width {
            let column = (position..self.trees.len()).step_by(self.width);

            compute_visible_distance(&self.trees, &mut column.clone(), &mut distance_up);
            compute_visible_distance(&self.trees, &mut column.clone().rev(), &mut distance_down);
        }

        for position in 0..self.height {
            let row = position * self.width..(position + 1) * self.width;

            compute_visible_distance(&self.trees, &mut row.clone(), &mut distance_left);
            compute_visible_distance(&self.trees, &mut row.clone().rev(), &mut distance_right);
        }

        izip!(
            &distance_up,
            &distance_down,
            &distance_left,
            &distance_right
        )
        .map(|(u, d, l, r)| u * d * l * r)
        .max()
        .unwrap_or(0)
        .to_string()
    }
}

#[cfg(test)]
mod test {
    use crate::SolutionBase;
    use std::fs;

    use super::*;

    fn get_solution() -> Solution {
        let data = fs::read_to_string("data/day_08_example.txt").unwrap();

        Solution::new(&data)
    }

    #[test]
    fn new() {
        let solution = get_solution();

        assert_eq!(
            solution,
            Solution {
                width: 5,
                height: 5,
                trees: vec![
                    3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
                ],
            }
        )
    }

    #[test]
    fn part_1() {
        let solution = get_solution();

        assert_eq!(solution.part_1(), "21");
    }

    #[test]
    fn part_2() {
        let solution = get_solution();

        assert_eq!(solution.part_2(), "8");
    }
}
