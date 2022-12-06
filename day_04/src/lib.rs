#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::ops::RangeInclusive;

#[must_use]
pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (first, second) = parse_section_assignment_pairs(line);
            let left_contains_right =
                first.start() <= second.start() && first.end() >= second.end();
            let right_contains_left =
                second.start() <= first.start() && second.end() >= first.end();
            left_contains_right || right_contains_left
        })
        .count()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let (first, second) = parse_section_assignment_pairs(line);
            first
                .into_iter()
                .any(|section_id| second.contains(&section_id))
            // overlap(&left_section, &right_section) || overlap(&right_section, &left_section)
        })
        .count()
}

fn parse_section_assignment_pairs(line: &str) -> (RangeInclusive<u8>, RangeInclusive<u8>) {
    let (left, right) = line.split_once(',').unwrap();
    (
        parse_section_assignment(left),
        parse_section_assignment(right),
    )
}

fn parse_section_assignment(input: &str) -> RangeInclusive<u8> {
    let (start, end) = input.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

// fn overlap(left: &RangeInclusive<u8>, right: &RangeInclusive<u8>) -> bool {
//     left.start() <= right.start() && left.end() <= right.start()
// }

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
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 599);
    }

    #[test]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 4);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 928);
    }
}
