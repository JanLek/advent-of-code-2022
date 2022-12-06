#![deny(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

#[must_use]
pub fn part_1(input: &str) -> usize {
    let buffer = input.as_bytes();
    find_unique_sequence(buffer, 4)
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let buffer = input.as_bytes();
    find_unique_sequence(buffer, 14)
}

fn find_unique_sequence(buffer: &[u8], length: usize) -> usize {
    for i in length - 1..buffer.len() {
        let sequence = &buffer[i + 1 - length..=i];
        let deduped: HashSet<u8> = HashSet::from_iter(sequence.iter().copied());
        if deduped.len() == sequence.len() {
            return i + 1;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 1_042);
    }

    #[test]
    fn part_2_sample_test() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 2_980);
    }
}
