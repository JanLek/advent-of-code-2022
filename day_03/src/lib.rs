#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

#[must_use]
pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first_compartment, second_compartment) = line.as_bytes().split_at(line.len() / 2);
            let common_item = *first_compartment
                .iter()
                .find(|item| second_compartment.contains(item))
                .unwrap();
            item_value(common_item)
        })
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    let rucksacks = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    rucksacks
        .chunks(3)
        .map(|group_rucksacks| {
            let common_item = *group_rucksacks[0]
                .iter()
                .find(|item| group_rucksacks[1].contains(item) && group_rucksacks[2].contains(item))
                .unwrap();
            item_value(common_item)
        })
        .sum()
}

fn item_value(item: u8) -> u32 {
    u32::from(match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 157);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 8_072);
    }

    #[test]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 70);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 2_567);
    }
}
