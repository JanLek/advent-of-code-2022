#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
};

#[must_use]
pub fn part_1(input: &str) -> usize {
    let head_motions = parse(input);
    simulate_rope::<2>(head_motions)
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let head_motions = parse(input);
    simulate_rope::<10>(head_motions)
}

fn parse(input: &str) -> impl Iterator<Item = (&str, u8)> {
    input.lines().map(|line| {
        let (direction, num_steps) = line.split_once(' ').unwrap();
        (direction, num_steps.parse::<u8>().unwrap())
    })
}

fn simulate_rope<'a, const L: usize>(head_motions: impl Iterator<Item = (&'a str, u8)>) -> usize {
    let mut rope: [(i16, i16); L] = [(0, 0); L];

    let mut tail_history = HashSet::new();
    tail_history.insert(rope[L - 1]);

    for (direction, num_steps) in head_motions {
        for _ in 0..num_steps {
            let head = &mut rope[0];
            match direction {
                "U" => head.1 += 1,
                "R" => head.0 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                _ => panic!(),
            };

            for i in 1..L {
                let previous_knot = rope[i - 1];
                let knot = &mut rope[i];
                if previous_knot.0.abs_diff(knot.0) > 1 || previous_knot.1.abs_diff(knot.1) > 1 {
                    match previous_knot.0.cmp(&knot.0) {
                        Greater => knot.0 += 1,
                        Equal => (),
                        Less => knot.0 -= 1,
                    }
                    match previous_knot.1.cmp(&knot.1) {
                        Greater => knot.1 += 1,
                        Equal => (),
                        Less => knot.1 -= 1,
                    }
                }
            }

            tail_history.insert(rope[L - 1]);
        }
    }

    tail_history.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const LARGE_SAMPLE_INPUT: &str = include_str!("../large-sample-input.txt");
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
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 1);
        assert_eq!(part_2(LARGE_SAMPLE_INPUT), 36);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 2_607);
    }
}
