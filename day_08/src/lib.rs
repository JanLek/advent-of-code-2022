#![deny(clippy::all, clippy::pedantic)]

use std::ops::Index;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let grove = Grove::from(input);
    (0..grove.num_rows)
        .flat_map(|row| (0..grove.row_length).map(move |column| (row, column)))
        .filter(|(row, column)| is_visible(&grove, *row, *column))
        .count()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    todo!()
}

fn is_visible(grove: &Grove, row: usize, column: usize) -> bool {
    let tree_height = grove[(row, column)];
    (0..row).all(|r| grove[(r, column)] < tree_height)
        || (row + 1..grove.num_rows).all(|r| grove[(r, column)] < tree_height)
        || (0..column).all(|c| grove[(row, c)] < tree_height)
        || (column + 1..grove.row_length).all(|c| grove[(row, c)] < tree_height)
}

struct Grove<'a> {
    trees: &'a [u8],
    row_length: usize,
    num_rows: usize,
}

impl<'a> From<&'a str> for Grove<'a> {
    fn from(s: &'a str) -> Self {
        let trees = s.as_bytes();
        let row_length = trees.iter().position(|&byte| byte == b'\n').unwrap();
        let num_rows = trees.len() / row_length;
        Self {
            trees,
            row_length,
            num_rows,
        }
    }
}

impl Index<(usize, usize)> for Grove<'_> {
    type Output = u8;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.trees[row * (self.row_length + 1) + column]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 1_845);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 0);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 0);
    }
}
