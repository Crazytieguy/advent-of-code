#![feature(array_windows)]
use std::{collections::HashMap, iter};

use itertools::{iterate, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn solution(data: &'static str, num_steps: usize) -> usize {
    let lines = data.lines().map(|line| line.as_bytes()).collect_vec();
    let rules: HashMap<[u8; 2], u8> = lines[2..]
        .iter()
        .map(|&line| match *line {
            [a, b, .., c] => ([a, b], c),
            _ => panic!(),
        })
        .collect();
    let pair_counts = iterate(lines[0].array_windows().copied().counts(), |prev| {
        prev.iter()
            .flat_map(|(&[a, c], &count)| {
                let b = rules[&[a, c]];
                [([a, b], count), ([b, c], count)]
            })
            .into_grouping_map()
            .sum()
    })
    .nth(num_steps)
    .unwrap();
    let elem_counts = pair_counts
        .into_iter()
        .map(|([c0, _], count)| (c0, count))
        .chain(iter::once((*lines[0].last().unwrap(), 1)))
        .into_grouping_map()
        .sum();
    let (min, max) = elem_counts.into_values().minmax().into_option().unwrap();
    max - min
}

fn part_a(data: &'static str) -> usize {
    solution(data, 10)
}

fn part_b(data: &'static str) -> usize {
    solution(data, 40)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 1588);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 2188189693529);
    }
}
