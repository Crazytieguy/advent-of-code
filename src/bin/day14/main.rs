#![feature(array_windows)]
use std::{collections::HashMap, iter};

use itertools::{self, Itertools, MinMaxResult};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

type Pair = [char; 2];
type Pairs = [Pair; 2];

fn parse(data: &'static str) -> (HashMap<Pair, usize>, HashMap<Pair, Pairs>) {
    let mut lines = data.lines();
    let initial_pair_counts = iter::once('^')
        .chain(lines.next().unwrap().chars())
        .chain(iter::once('$'))
        .tuple_windows()
        .map(|(c0, c1)| [c0, c1])
        .counts();
    lines.next();
    let rules = lines
        .map(|line| {
            let (pair_0, pair_1, insert) = line
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .collect_tuple()
                .unwrap();
            ([pair_0, pair_1], [[pair_0, insert], [insert, pair_1]])
        })
        .collect();
    (initial_pair_counts, rules)
}

fn step(pair_counts: &HashMap<Pair, usize>, rules: &HashMap<Pair, Pairs>) -> HashMap<Pair, usize> {
    pair_counts
        .iter()
        .flat_map(|(&p, &count)| {
            rules.get(&p).map_or_else(
                || vec![(p, count)],
                |&[p0, p1]| vec![(p0, count), (p1, count)],
            )
        })
        .into_grouping_map()
        .sum()
}

fn pair_counts_to_elem_counts(pair_counts: &HashMap<Pair, usize>) -> HashMap<char, usize> {
    let mut counts = pair_counts
        .iter()
        .flat_map(|(&[c0, c1], &count)| [(c0, count), (c1, count)])
        .filter(|(c, _)| !matches!(c, '^' | '$'))
        .into_grouping_map()
        .sum();
    counts.values_mut().for_each(|count| *count /= 2);
    counts
}

fn solution(data: &'static str, num_steps: usize) -> usize {
    let (pair_counts, rules) = parse(data);
    let pair_counts = itertools::iterate(pair_counts, |pair_counts| step(pair_counts, &rules))
        .nth(num_steps)
        .unwrap();
    let elem_counts = pair_counts_to_elem_counts(&pair_counts);
    match elem_counts.into_values().minmax() {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => unreachable!(),
    }
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
