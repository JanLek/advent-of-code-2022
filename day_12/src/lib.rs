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
    find_shortest_route(&map, start, end).unwrap()
}

#[must_use]
pub fn part_2(input: &[u8]) -> u32 {
    let map = HeightMap::from(input);
    let end = map.find(b'E');
    map.find_lowest_points()
        .filter_map(|start| find_shortest_route(&map, start, end))
        .min()
        .unwrap()
}

fn find_shortest_route(map: &HeightMap, start: Point, end: Point) -> Option<u32> {
    let mut cache = Cache::new(map.num_rows, map.num_columns);
    cache[start] = Some(0);
    let mut queue = VecDeque::from([start]);

    while cache[end].is_none() {
        if let Some(point) = queue.pop_back() {
            let height = map[point];
            let num_steps = cache[point].unwrap();
            for next_point in map.surrounding_points(point) {
                if cache[next_point].is_none() && map[next_point] <= height + 1 {
                    cache[next_point] = Some(num_steps + 1);
                    queue.push_front(next_point);
                }
            }
        } else {
            return None;
        }
    }

    Some(cache[end].unwrap())
}

type Point = (usize, usize);

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

impl<'a> Index<Point> for HeightMap<'a> {
    type Output = u8;

    fn index(&self, (row, column): Point) -> &Self::Output {
        match &self.data[row * (self.num_columns + 1) + column] {
            height @ b'a'..=b'z' => height,
            b'S' => &b'a',
            b'E' => &b'z',
            _ => panic!(),
        }
    }
}

impl HeightMap<'_> {
    fn find(&self, byte: u8) -> Point {
        (0..self.num_rows)
            .flat_map(|row| (0..self.num_columns).map(move |column| (row, column)))
            .find(|&(row, column)| self.data[row * (self.num_columns + 1) + column] == byte)
            .unwrap()
    }

    fn find_lowest_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.num_rows)
            .flat_map(|row| (0..self.num_columns).map(move |column| (row, column)))
            .filter(|&(row, column)| self.data[row * (self.num_columns + 1) + column] == b'a')
    }

    fn surrounding_points(&self, (row, column): Point) -> impl Iterator<Item = Point> + '_ {
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

impl Index<Point> for Cache {
    type Output = Option<u32>;

    fn index(&self, (row, column): Point) -> &Self::Output {
        &self.data[row * self.num_columns + column]
    }
}

impl IndexMut<Point> for Cache {
    fn index_mut(&mut self, (row, column): Point) -> &mut Self::Output {
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
        assert_eq!(part_1(INPUT), 350);
    }

    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2(SAMPLE_INPUT), 29);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 349);
    }
}
