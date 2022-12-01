const INPUT: &str = include_str!("../input.txt");

pub fn part_1(input: &str) -> u32 {
    todo!()
}

pub fn part_2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 42);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 42);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 42);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 42);
    }
}
