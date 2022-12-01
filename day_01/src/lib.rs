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
    let mut max = 0;
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            current = 0;
            continue;
        }
        
        let calories: u32 = line.parse().unwrap();
        current += calories;

        if current > max {
            max = current;
        }
    }

    max
}

fn find_top_3_max_calories(input: &str) -> u32 {
    let mut elves_snacks = Vec::new();
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            elves_snacks.push(current);
            current = 0;
            continue;
        }
        
        let calories: u32 = line.parse().unwrap();
        current += calories;
    }
    elves_snacks.push(current);

    elves_snacks.sort_unstable();

    elves_snacks.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(find_max_calories(input), 24_000);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(), 70698);
    }

    #[test]
    fn test_part_2_sample() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        assert_eq!(find_top_3_max_calories(input), 45000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(), 206_643);
    }
}
