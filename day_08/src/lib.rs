#![deny(clippy::all, clippy::pedantic)]

use std::ops::Index;

#[must_use]
pub fn part_1(input: &str) -> usize {
    Grove::from(input).count_visible_trees()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    Grove::from(input).max_scenic_score()
}

struct Grove<'a> {
    trees: &'a [u8],
    num_columns: usize,
    num_rows: usize,
}

impl Grove<'_> {
    fn all_coordinates(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.num_rows).flat_map(|row| (0..self.num_columns).map(move |column| (row, column)))
    }

    fn count_visible_trees(&self) -> usize {
        self.all_coordinates()
            .filter(|&(row, column)| {
                let tree_height = self[(row, column)];
                // Check in every direction if there are no higher trees blocking the view.
                (0..row).all(|r| self[(r, column)] < tree_height)
                    || (row + 1..self.num_rows).all(|r| self[(r, column)] < tree_height)
                    || (0..column).all(|c| self[(row, c)] < tree_height)
                    || (column + 1..self.num_columns).all(|c| self[(row, c)] < tree_height)
            })
            .count()
    }

    fn max_scenic_score(&self) -> usize {
        self.all_coordinates()
            .map(|(row, column)| {
                // Count the trees in line of sight in every direction.
                self.count_in_line_of_sight((0..=row).rev().map(|r| (r, column)))
                    * self.count_in_line_of_sight((row..self.num_rows).map(|r| (r, column)))
                    * self.count_in_line_of_sight((0..=column).rev().map(|c| (row, c)))
                    * self.count_in_line_of_sight((column..self.num_columns).map(|c| (row, c)))
            })
            .max()
            .unwrap()
    }

    fn count_in_line_of_sight(&self, mut points: impl Iterator<Item = (usize, usize)>) -> usize {
        let tree_height = self[points.next().unwrap()];
        let mut count = 0;
        for (row, column) in points {
            count += 1;
            if self[(row, column)] >= tree_height {
                break;
            }
        }
        count
    }
}

impl<'a> From<&'a str> for Grove<'a> {
    fn from(s: &'a str) -> Self {
        let trees = s.as_bytes();
        let num_columns = trees.iter().position(|&byte| byte == b'\n').unwrap();
        let num_rows = trees.len() / num_columns;
        Self {
            trees,
            num_columns,
            num_rows,
        }
    }
}

impl Index<(usize, usize)> for Grove<'_> {
    type Output = u8;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.trees[row * (self.num_columns + 1) + column]
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
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 8);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 230_112);
    }
}
