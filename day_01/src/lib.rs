#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

const INPUT: &str = include_str!("input.txt");

#[must_use]
pub fn part_1() -> u32 {
    find_max_calories(INPUT)
}

#[must_use]
pub fn part_2() -> u32 {
    find_top_3_max_calories(INPUT)
}

fn find_max_calories(input: &str) -> u32 {
    calories_per_elf(input).max().unwrap()
}

fn find_top_3_max_calories(input: &str) -> u32 {
    let mut counts: Vec<_> = calories_per_elf(input).collect();
    counts.sort_unstable();
    counts.into_iter().rev().take(3).sum()
}

fn calories_per_elf(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split("\n\n")
        .map(|i| i.lines().map(parse_calories).sum())
}

// Custom parse function for minor speed improvement
fn parse_calories(line: &str) -> u32 {
    let mut magnitude = 1;
    let mut result = 0;
    for digit in line.bytes().rev() {
        result += u32::from(digit - b'0') * magnitude;
        magnitude *= 10;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");

    #[test]
    fn test_part_1_sample() {
        assert_eq!(find_max_calories(SAMPLE_INPUT), 24_000);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), 70698);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(find_top_3_max_calories(SAMPLE_INPUT), 45000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), 206_643);
    }
}
