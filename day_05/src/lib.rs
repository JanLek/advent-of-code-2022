#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

#[must_use]
pub fn part_1(input: &str) -> String {
    let (mut stacks, steps) = parse_input(input);
    crate_mover_9000(steps, &mut stacks);
    top_items(stacks)
}

#[must_use]
pub fn part_2(input: &str) -> String {
    let (mut stacks, steps) = parse_input(input);
    crate_mover_9001(steps, &mut stacks);
    top_items(stacks)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(u8, usize, usize)>) {
    let (stacks_input, steps_input) = input.split_once("\n\n").unwrap();

    let mut stacks_lines: Vec<&str> = stacks_input.lines().collect();
    let last_line = stacks_lines.pop().unwrap();
    let num_stacks: usize = (last_line.as_bytes()[last_line.len() - 2] - b'0') as usize;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    for line in stacks_lines.into_iter().rev() {
        for i in 0..num_stacks {
            match line.as_bytes()[1 + i * 4] {
                byte @ b'A'..=b'Z' => stacks[i].push(byte as char),
                b' ' => (),
                byte => panic!(
                    "Unexpected character: '{}' in line: \"{}\"",
                    byte as char, line
                ),
            }
        }
    }

    let steps: Vec<(u8, usize, usize)> = steps_input
        .lines()
        .map(|mut line| {
            line = line.strip_prefix("move ").unwrap();
            let (num_items, line) = line.split_once(" from ").unwrap();
            let (from, to) = line.split_once(" to ").unwrap();
            (
                num_items.parse().unwrap(),
                from.parse().unwrap(),
                to.parse().unwrap(),
            )
        })
        .collect();

    (stacks, steps)
}

fn crate_mover_9000(steps: Vec<(u8, usize, usize)>, stacks: &mut Vec<Vec<char>>) {
    for (num_items, from, to) in steps {
        for _ in 0..num_items {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }
}

fn crate_mover_9001(steps: Vec<(u8, usize, usize)>, stacks: &mut Vec<Vec<char>>) {
    let mut crane_stack = Vec::new();
    for (num_items, from, to) in steps {
        for _ in 0..num_items {
            crane_stack.push(stacks[from - 1].pop().unwrap());
        }
        while let Some(item) = crane_stack.pop() {
            stacks[to - 1].push(item);
        }
    }
}

fn top_items(stacks: Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../sample-input.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part_1_sample_test() {
        assert_eq!(part_1(SAMPLE_INPUT), String::from("CMZ"));
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), String::from("ZRLJGSCTR"));
    }

    #[test]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), String::from("MCD"));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), String::from("PRTTGRFPB"));
    }
}
