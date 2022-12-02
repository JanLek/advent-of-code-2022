use HandShape::*;
use Outcome::*;

pub fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let opponents_choice = HandShape::from(line.as_bytes()[0]);
            let my_choice = HandShape::from(line.as_bytes()[2]);
            my_choice.score() + my_choice.play_round(opponents_choice).score()
        })
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let opponents_choice = HandShape::from(line.as_bytes()[0]);
            let outcome = Outcome::from(line.as_bytes()[2]);
            let my_choice = match outcome {
                Loss => opponents_choice.beats(),
                Draw => opponents_choice,
                Win => opponents_choice.beats().beats(),
            };
            my_choice.score() + outcome.score()
        })
        .sum()
}

#[derive(Clone, Copy, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl From<u8> for HandShape {
    fn from(b: u8) -> Self {
        match b {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            b'X' => Rock,
            b'Y' => Paper,
            b'Z' => Scissors,
            _ => panic!(),
        }
    }
}

impl HandShape {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn play_round(&self, other: Self) -> Outcome {
        if self == &other {
            Draw
        } else if self.beats() == other {
            Win
        } else {
            Loss
        }
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl From<u8> for Outcome {
    fn from(b: u8) -> Self {
        match b {
            b'X' => Loss,
            b'Y' => Draw,
            b'Z' => Win,
            _ => panic!(),
        }
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Win => 6,
            Draw => 3,
            Loss => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 15);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 13_809);
    }

    #[test]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 12);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 12_316);
    }
}
