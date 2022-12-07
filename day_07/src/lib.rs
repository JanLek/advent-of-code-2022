#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

#[must_use]
pub fn part_1(input: &str) -> u32 {
    calculate_directory_sizes(input)
        .iter()
        .filter_map(|directory| {
            if directory.size <= 100_000 {
                Some(directory.size)
            } else {
                None
            }
        })
        .sum()
}

#[must_use]
pub fn part_2(input: &str) -> u32 {
    let directories = calculate_directory_sizes(input);
    let needed_space = 30_000_000 - (70_000_000 - directories[0].size);
    directories
        .iter()
        .filter_map(|directory| {
            if directory.size >= needed_space {
                Some(directory.size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn calculate_directory_sizes(input: &str) -> Vec<Directory> {
    let mut directories = vec![Directory::new("/", 0)];
    let mut current_directory = 0;

    for line in input.lines().skip(1) {
        if line == "$ cd .." {
            let parent = directories[current_directory].parent;
            directories[parent].size += directories[current_directory].size;
            current_directory = parent;
        } else if let Some(child) = line.strip_prefix("$ cd ") {
            current_directory = directories
                .iter()
                .position(|directory| {
                    directory.parent == current_directory && directory.name == child
                })
                .unwrap();
        } else if let Some(child) = line.strip_prefix("dir ") {
            directories.push(Directory::new(child, current_directory));
        } else if line != "$ ls" {
            directories[current_directory].size +=
                line.split_once(' ').unwrap().0.parse::<u32>().unwrap();
        }
    }

    while current_directory != 0 {
        let parent = directories[current_directory].parent;
        directories[parent].size += directories[current_directory].size;
        current_directory = parent;
    }

    directories
}

struct Directory<'a> {
    name: &'a str,
    parent: usize,
    size: u32,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, parent: usize) -> Self {
        Self {
            name,
            parent,
            size: 0,
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
        assert_eq!(part_1(SAMPLE_INPUT), 95_437);
    }

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 1_443_806);
    }

    #[test]
    fn part_2_sample_test() {
        assert_eq!(part_2(SAMPLE_INPUT), 24_933_642);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 942_298);
    }
}
