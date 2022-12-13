#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::{
        Ordering,
        Ordering::{Equal, Greater, Less},
    },
    str::FromStr,
};
use ListOrInteger::{Integer, List};

#[must_use]
pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|i| {
            let (left, right) = i.split_once('\n').unwrap();
            (
                left.parse::<ListOrInteger>().unwrap(),
                right.parse().unwrap(),
            )
        })
        .enumerate()
        .filter_map(|(index, (left, right))| match left.cmp(&right) {
            Less => Some(index + 1),
            Equal => panic!(),
            Greater => None,
        })
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();
    packets.push(ListOrInteger::divider(2));
    packets.push(ListOrInteger::divider(6));
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| packet.is_divider().then_some(index + 1))
        .product()
}

#[derive(Debug, Eq, PartialEq)]
enum ListOrInteger {
    List(Vec<ListOrInteger>),
    Integer(u32),
}

impl ListOrInteger {
    fn divider(integer: u32) -> Self {
        List(vec![List(vec![Integer(integer)])])
    }

    fn is_divider(&self) -> bool {
        self == &Self::divider(2) || self == &Self::divider(6)
    }
}

impl PartialOrd for ListOrInteger {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListOrInteger {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Integer(l), Integer(r)) => l.cmp(r),
            (List(l), List(r)) => cmp_lists(l, r),
            (Integer(l), List(r)) => cmp_lists(&vec![Integer(*l)], r),
            (List(l), Integer(r)) => cmp_lists(l, &vec![Integer(*r)]),
        }
    }
}

fn cmp_lists(left: &Vec<ListOrInteger>, right: &Vec<ListOrInteger>) -> Ordering {
    let left_len = left.len();
    let right_len = right.len();

    for (left_value, right_value) in left.iter().zip(right.iter()) {
        match left_value.cmp(right_value) {
            Less => return Less,
            Equal => (),
            Greater => return Greater,
        }
    }

    left_len.cmp(&right_len)
}

impl FromStr for ListOrInteger {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(parse_list_or_integer(input).unwrap().1)
    }
}

fn parse_list(input: &str) -> Option<(usize, Vec<ListOrInteger>)> {
    input.strip_prefix('[')?;
    let mut i = 1;

    let mut list = Vec::new();
    while let Some((num_chars, list_or_integer)) = parse_list_or_integer(&input[i..]) {
        list.push(list_or_integer);
        i += num_chars;
        if input[i..].starts_with(',') {
            i += 1;
        }
    }

    input[i..].strip_prefix(']')?;
    i += 1;

    Some((i, list))
}

fn parse_integer(input: &str) -> Option<(usize, u32)> {
    let num_digits = input.chars().take_while(char::is_ascii_digit).count();
    if let Ok(digit) = input[0..num_digits].parse() {
        Some((num_digits, digit))
    } else {
        None
    }
}

fn parse_list_or_integer(input: &str) -> Option<(usize, ListOrInteger)> {
    if let Some((num_chars, list)) = parse_list(input) {
        Some((num_chars, List(list)))
    } else if let Some((num_chars, integer)) = parse_integer(input) {
        Some((num_chars, Integer(integer)))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");
    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(SAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 5_292);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(SAMPLE_INPUT), 140);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 23_868);
    }
}
