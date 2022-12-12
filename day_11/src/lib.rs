#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::{collections::VecDeque, str::FromStr, usize};

#[must_use]
pub fn part_1(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|i| i.parse().unwrap()).collect();
    simulate_rounds(&mut monkeys, 20, 3);
    level_of_monkey_business(&monkeys)
}

#[must_use]
pub fn part_2(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|i| i.parse().unwrap()).collect();
    simulate_rounds(&mut monkeys, 10_000, 1);
    level_of_monkey_business(&monkeys)
}

fn simulate_rounds(monkeys: &mut Vec<Monkey>, num_rounds: usize, worry_divisor: u64) {
    let common_divisor = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            for _ in 0..monkeys[i].items.len() {
                let monkey = &mut monkeys[i];
                monkey.inspect_item();
                monkey.reduce_worry_level(worry_divisor, common_divisor);
                let (item, to) = monkey.throw_item();
                monkeys[to].items.push_back(item);
            }
        }
    }
}

fn level_of_monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut inspection_counts: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspections).collect();
    inspection_counts.sort_unstable();
    inspection_counts.iter().rev().take(2).product()
}

struct Monkey {
    items: VecDeque<u64>,
    operation_left_operand: Option<u64>,
    operator: fn(u64, u64) -> u64,
    operation_right_operand: Option<u64>,
    test_divisor: u64,
    test_true_throw: usize,
    test_false_throw: usize,
    inspections: u64,
}

impl Monkey {
    fn inspect_item(&mut self) {
        let item = self.items[0];
        self.items[0] = (self.operator)(
            self.operation_left_operand.unwrap_or(item),
            self.operation_right_operand.unwrap_or(item),
        );
        self.inspections += 1;
    }

    fn reduce_worry_level(&mut self, divisor: u64, common_divisor: u64) {
        self.items[0] /= divisor;
        self.items[0] %= common_divisor;
    }

    fn throw_item(&mut self) -> (u64, usize) {
        let item = self.items.pop_front().unwrap();
        if item % self.test_divisor == 0 {
            (item, self.test_true_throw)
        } else {
            (item, self.test_false_throw)
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines().skip(1);
        let items: VecDeque<u64> = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let mut operation_parts = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .split(' ');
        let operation_left_operand = operation_parts.next().unwrap().parse::<u64>().ok();
        let operator = match operation_parts.next().unwrap() {
            "+" => std::ops::Add::add,
            "-" => std::ops::Sub::sub,
            "*" => std::ops::Mul::mul,
            _ => panic!(),
        };
        let operation_right_operand = operation_parts.next().unwrap().parse::<u64>().ok();
        let test_divisor = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let test_true_throw = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let test_false_throw = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Ok(Self {
            items,
            operation_left_operand,
            operator,
            operation_right_operand,
            test_divisor,
            test_true_throw,
            test_false_throw,
            inspections: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), 10_605);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 100_345);
    }

    #[test]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 2_713_310_158);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 28_537_348_205);
    }
}
