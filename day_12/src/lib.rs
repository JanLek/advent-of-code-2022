#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

#[must_use]
pub fn part_1(input: &[u8]) -> u32 {
    let map = HeightMap::from(input);
    let start = map.find(b'S');
    let end = map.find(b'E');
    let mut cache = Cache::new(map.num_rows, map.num_columns);
    cache[start] = Some(0);
    let mut queue = VecDeque::from([start]);

    while cache[end].is_none() {
        let point = queue.pop_back().unwrap();
        let height = map[point];
        let num_steps = cache[point].unwrap();
        for next_point in map.surrounding_points(point) {
            if cache[next_point].is_none() && map[next_point] <= height + 1 {
                cache[next_point] = Some(num_steps + 1);
                queue.push_front(next_point);
            }
        }
    }

    cache[end].unwrap()
}

struct HeightMap<'a> {
    num_columns: usize,
    num_rows: usize,
    data: &'a [u8],
}

impl<'a> From<&'a [u8]> for HeightMap<'a> {
    fn from(data: &'a [u8]) -> Self {
        let num_columns = data.iter().position(|&byte| byte == b'\n').unwrap();
        let num_rows = data.len() / num_columns;
        Self {
            num_columns,
            num_rows,
            data,
        }
    }
}

impl<'a> Index<(usize, usize)> for HeightMap<'a> {
    type Output = u8;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        match &self.data[row * (self.num_columns + 1) + column] {
            height @ b'a'..=b'z' => height,
            b'S' => &b'a',
            b'E' => &b'z',
            _ => panic!(),
        }
    }
}

impl HeightMap<'_> {
    fn find(&self, byte: u8) -> (usize, usize) {
        (0..self.num_rows)
            .flat_map(|row| (0..self.num_columns).map(move |column| (row, column)))
            .find(|&(row, column)| self.data[row * (self.num_columns + 1) + column] == byte)
            .unwrap()
    }

    fn surrounding_points(
        &self,
        (row, column): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        row.checked_sub(1)
            .into_iter()
            .chain(Some(row + 1).filter(|&r| r < self.num_rows).into_iter())
            .map(move |r| (r, column))
            .chain(
                column
                    .checked_sub(1)
                    .into_iter()
                    .chain(
                        Some(column + 1)
                            .filter(|&c| c < self.num_columns)
                            .into_iter(),
                    )
                    .map(move |c| (row, c)),
            )
    }
}

struct Cache {
    num_columns: usize,
    data: Vec<Option<u32>>,
}

impl Cache {
    fn new(num_rows: usize, num_columns: usize) -> Self {
        Self {
            num_columns,
            data: vec![None; num_rows * num_columns],
        }
    }
}

impl Index<(usize, usize)> for Cache {
    type Output = Option<u32>;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        &self.data[row * self.num_columns + column]
    }
}

impl IndexMut<(usize, usize)> for Cache {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.num_columns + column]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &[u8] = include_bytes!("../sample-input.txt");
    const INPUT: &[u8] = include_bytes!("../input.txt");

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1(SAMPLE_INPUT), 31);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 31);
    }
}
