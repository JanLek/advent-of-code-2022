#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

#[must_use]
pub fn part_1(input: &str) -> usize {
    todo!()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const LARGE_SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    #[ignore = "TODO"]
    fn part_1_sample_test() {
        assert_eq!(part_1(SMALL_SAMPLE_INPUT), 0);
        assert_eq!(part_1(LARGE_SAMPLE_INPUT), 13140);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 0);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_sample_test() {
        assert_eq!(part_2(SMALL_SAMPLE_INPUT), 0);
        assert_eq!(part_2(LARGE_SAMPLE_INPUT), 0);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 0);
    }
}
