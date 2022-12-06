#![deny(clippy::all, clippy::pedantic)]

#[must_use]
pub fn part_1(input: &str) -> usize {
    find_unique_sequence(input.as_bytes(), 4)
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    find_unique_sequence(input.as_bytes(), 14)
}

fn find_unique_sequence(buffer: &[u8], length: usize) -> usize {
    buffer.windows(length).position(all_bytes_unique).unwrap() + length
}

fn all_bytes_unique(sequence: &[u8]) -> bool {
    sequence
        .iter()
        .enumerate()
        .all(|(i, byte)| !sequence[i + 1..].contains(byte))
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
