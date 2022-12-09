#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
};

#[must_use]
pub fn part_1(input: &str) -> usize {
    let head_motions = input.lines().map(|line| {
        let (direction, num_steps) = line.split_once(' ').unwrap();
        (direction, num_steps.parse::<u8>().unwrap())
    });

    let mut head: (i16, i16) = (0, 0);
    let mut tail: (i16, i16) = (0, 0);
    let mut tail_history = HashSet::new();
    tail_history.insert(tail);

    for (direction, num_steps) in head_motions {
        for _ in 0..num_steps {
            match direction {
                "U" => head.1 += 1,
                "R" => head.0 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                _ => panic!(),
            };

            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                match head.0.cmp(&tail.0) {
                    Greater => tail.0 += 1,
                    Equal => (),
                    Less => tail.0 -= 1,
                }
                match head.1.cmp(&tail.1) {
                    Greater => tail.1 += 1,
                    Equal => (),
                    Less => tail.1 -= 1,
                }

                tail_history.insert(tail);
            }
        }
    }

    tail_history.len()
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
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 13);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 6_376);
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
