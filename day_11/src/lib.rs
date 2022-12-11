#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

#[must_use]
pub fn part_1(input: &str) -> u32 {
    todo!()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    #[ignore]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 15);
    }

    #[test]
    #[ignore]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 13_809);
    }

    #[test]
    #[ignore]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 12);
    }

    #[test]
    #[ignore]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 12_316);
    }
}
