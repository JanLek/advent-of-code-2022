#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::{
    iter::{once, repeat},
    str::FromStr,
};
use CpuState::{Adding, Ready};
use Instruction::{AddX, Noop};

#[must_use]
pub fn part_1(input: &str) -> i32 {
    let mut result = 0;

    let instructions = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap());
    let mut cpu = Cpu::new(instructions);

    for steps in once(20).chain(repeat(40)).take(6) {
        for _ in 0..(steps - 1) {
            cpu.tick();
        }
        let signal_strenth = (cpu.cycles + 1) * cpu.x_register;
        result += signal_strenth;
        cpu.tick();
    }

    result
}

#[must_use]
pub fn part_2(input: &str) -> usize {
    todo!()
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Noop)
        } else {
            Ok(AddX(s.strip_prefix("addx ").unwrap().parse().unwrap()))
        }
    }
}

#[derive(Debug)]
struct Cpu<I>
where
    I: Iterator<Item = Instruction>,
{
    instructions: I,
    cycles: i32,
    x_register: i32,
    state: CpuState,
}

impl<I> Cpu<I>
where
    I: Iterator<Item = Instruction>,
{
    fn new(instructions: I) -> Self {
        Self {
            instructions,
            cycles: 0,
            x_register: 1,
            state: Ready,
        }
    }

    fn tick(&mut self) {
        match self.state {
            Ready => {
                let instruction = self.instructions.next().unwrap();
                self.state = match instruction {
                    Noop => Ready,
                    AddX(value) => Adding(value),
                };
            }
            Adding(value) => {
                self.x_register += value;
                self.state = Ready;
            }
        }
        self.cycles += 1;
    }
}

#[derive(Debug)]
enum CpuState {
    Ready,
    Adding(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMLE_INPUT), 13140);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 14_160);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMLE_INPUT), 0);
    }

    #[test]
    #[ignore = "TODO"]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 0);
    }
}
