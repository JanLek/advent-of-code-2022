#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::{
    fmt::{Display, Formatter, Result as FormatResult},
    iter::{once, repeat},
    str::FromStr,
};
use CpuState::{Adding, Done, Ready};
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
pub fn part_2(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap());
    let mut cpu = Cpu::new(instructions);
    let mut crt = Crt::new();

    while !cpu.is_done() {
        crt.draw_pixel(cpu.cycles, cpu.x_register);
        cpu.tick();
    }

    format!("{}", crt)
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
                let instruction = self.instructions.next();
                self.state = match instruction {
                    Some(Noop) => Ready,
                    Some(AddX(value)) => Adding(value),
                    None => Done,
                };
            }
            Adding(value) => {
                self.x_register += value;
                self.state = Ready;
            }
            Done => {}
        }
        self.cycles += 1;
    }

    fn is_done(&self) -> bool {
        matches!(self.state, Done)
    }
}

#[derive(Debug)]
enum CpuState {
    Ready,
    Adding(i32),
    Done,
}

struct Crt([u8; 240]);

impl Crt {
    fn new() -> Self {
        Self([b'.'; 240])
    }

    fn draw_pixel(&mut self, cycle: i32, x_register: i32) {
        let adjusted_register = x_register + 40 * (cycle / 40);
        if (cycle - 1..=cycle + 1).contains(&adjusted_register) {
            self.0[usize::try_from(cycle).unwrap()] = b'#';
        }
    }
}

impl Display for Crt {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        for row in self.0.chunks(40) {
            writeln!(formatter, "{}", std::str::from_utf8(row).unwrap())?;
        }
        Ok(())
    }
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
    fn part_2_sample_test() {
        assert_eq!(
            part_2(SAMLE_INPUT),
            String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            )
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            part_2(INPUT),
            String::from(
                "###....##.####.###..###..####.####..##..
#..#....#.#....#..#.#..#.#....#....#..#.
#..#....#.###..#..#.#..#.###..###..#....
###.....#.#....###..###..#....#....#....
#.#..#..#.#....#.#..#....#....#....#..#.
#..#..##..####.#..#.#....####.#.....##..
"
            )
        );
    }
}
