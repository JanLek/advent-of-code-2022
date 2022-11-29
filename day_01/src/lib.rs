#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

const _: &str = include_str!("input.txt");

#[must_use]
pub fn part_1() -> i32 {
    123
}

#[must_use]
pub fn part_2() -> i32 {
    123
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "todo"]
    fn test_part_1() {
        assert_eq!(part_1(), 42);
    }

    #[test]
    #[ignore = "todo"]
    fn test_part_2() {
        assert_eq!(part_2(), 42);
    }
}
