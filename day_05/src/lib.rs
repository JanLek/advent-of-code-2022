#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::ops::RangeInclusive;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let (stacks, steps) = parse_input(input);
    todo!()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    todo!()
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<(u8, usize, usize)) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 2);
    }

    #[test]
    #[ignore]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 599);
    }

    #[test]
    #[ignore]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 4);
    }

    #[test]
    #[ignore]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 928);
    }
}
