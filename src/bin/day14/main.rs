#![feature(array_windows)]
use std::collections::HashMap;

use itertools::{self, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

type Pair = [char; 2];

fn parse(data: &'static str) -> (HashMap<Pair, usize>, HashMap<Pair, char>) {
    let lines = data
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut initial_pair_counts = lines[0].array_windows().copied().counts();
    initial_pair_counts.insert([*lines[0].last().unwrap(), '$'], 1);
    let rules = lines[2..]
        .iter()
        .map(|line| ([line[0], line[1]], line[6]))
        .collect();
    (initial_pair_counts, rules)
}

fn solution(data: &'static str, num_steps: usize) -> usize {
    let (pair_counts, rules) = parse(data);
    let pair_counts = itertools::iterate(pair_counts, |pair_counts| {
        pair_counts
            .iter()
            .flat_map(|(&[c0, c1], &count)| {
                if let Some(&insert) = rules.get(&[c0, c1]) {
                    vec![([c0, insert], count), ([insert, c1], count)]
                } else {
                    vec![([c0, c1], count)]
                }
            })
            .into_grouping_map()
            .sum()
    })
    .nth(num_steps)
    .unwrap();
    let elem_counts = pair_counts
        .iter()
        .map(|(&[c0, _], &count)| (c0, count))
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
