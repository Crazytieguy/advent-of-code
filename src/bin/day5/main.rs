use std::{
    iter::{repeat, Chain, Rev},
    ops::RangeInclusive,
};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl From<&'static str> for Vent {
    fn from(s: &'static str) -> Self {
        let (x1, y1, x2, y2) = s
            .split(" -> ")
            .flat_map(|point| point.split(','))
            .map(|coord| coord.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self { x1, y1, x2, y2 }
    }
}

fn route1d(start: i32, stop: i32) -> Chain<RangeInclusive<i32>, Rev<RangeInclusive<i32>>> {
    (start..=stop).chain((stop..=start).rev())
}

fn solution(data: &'static str, calc_diagonal: impl Fn(Vent) -> Vec<(i32, i32)>) -> usize {
    data.lines()
        .map(Vent::from)
        .flat_map(|Vent { x1, y1, x2, y2 }| {
            if x1 == x2 {
                repeat(x1).zip(route1d(y1, y2)).collect()
            } else if y1 == y2 {
                route1d(x1, x2).zip(repeat(y1)).collect()
            } else {
                calc_diagonal(Vent { x1, y1, x2, y2 })
            }
        })
        .counts()
        .values()
        .filter(|&&v| v >= 2)
        .count()
}

fn part_a(data: &'static str) -> usize {
    solution(data, |_| vec![])
}

fn part_b(data: &'static str) -> usize {
    solution(data, |Vent { x1, y1, x2, y2 }| {
        route1d(x1, x2).zip(route1d(y1, y2)).collect()
    })
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
