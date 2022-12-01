use std::ops::RangeInclusive;

use itertools::Itertools;
use ndarray::{s, Array2};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> impl Iterator<Item = (usize, usize, usize, usize)> {
    data.lines().map(|line| {
        line.split(" -> ")
            .flat_map(|point| point.split(','))
            .map(|coord| coord.parse().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

fn range(c1: usize, c2: usize) -> RangeInclusive<usize> {
    c1.min(c2)..=c1.max(c2)
}

fn solution(data: &'static str, calc_diagonal: bool) -> usize {
    let mut grid = Array2::zeros([1000, 1000]);
    for (x1, y1, x2, y2) in parse(data) {
        if x1 == x2 {
            let mut points = grid.slice_mut(s![x1, range(y1, y2)]);
            points += 1;
        } else if y1 == y2 {
            let mut points = grid.slice_mut(s![range(x1, x2), y1]);
            points += 1;
        } else if calc_diagonal {
            let step_x = if x1 < x2 { 1 } else { -1 };
            let step_y = if y1 < y2 { 1 } else { -1 };
            let mut slice = grid.slice_mut(s![
                range(x1, x2);step_x,
                range(y1, y2);step_y
            ]);
            let mut points = slice.diag_mut();
            points += 1;
        }
    }
    grid.fold(0, |acc, &cur| acc + (cur >= 2) as usize)
}

fn part_a(data: &'static str) -> usize {
    solution(data, false)
}

fn part_b(data: &'static str) -> usize {
    solution(data, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 5);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 12);
    }
}
