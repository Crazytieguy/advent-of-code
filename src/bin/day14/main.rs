#![feature(array_windows)]
use std::collections::HashMap;

use itertools::{iterate, Itertools, MinMaxResult};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

type Pair = [char; 2];
type Pairs = [Pair; 2];

fn parse(data: &'static str) -> (HashMap<Pair, usize>, HashMap<Pair, Pairs>) {
    let mut lines = data.lines();
    let template = lines
        .next()
        .unwrap()
        .chars()
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
    (template, rules)
}

fn step(pair_counts: &HashMap<Pair, usize>, rules: &HashMap<Pair, Pairs>) -> HashMap<Pair, usize> {
    let mut next_pair_counts = HashMap::new();
    for (&pair, &count) in pair_counts {
        if let Some(&[pair_0, pair_1]) = rules.get(&pair) {
            *next_pair_counts.entry(pair_0).or_default() += count;
            *next_pair_counts.entry(pair_1).or_default() += count;
        } else {
            *next_pair_counts.entry(pair).or_default() += count;
        }
    }
    next_pair_counts
}

fn pair_counts_to_elem_counts(pair_counts: &HashMap<Pair, usize>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for (&[c0, c1], &count) in pair_counts {
        *counts.entry(c0).or_default() += count;
        *counts.entry(c1).or_default() += count;
    }
    for count in counts.values_mut() {
        *count = *count / 2 + *count % 2;
    }
    counts
}

fn part_a(data: &'static str) -> usize {
    let (pair_counts, rules) = parse(data);
    let pair_counts = iterate(pair_counts, |pair_counts| step(pair_counts, &rules))
        .nth(10)
        .unwrap();

    match pair_counts_to_elem_counts(&pair_counts)
        .into_values()
        .minmax()
    {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => panic!(),
    }
}

fn part_b(data: &'static str) -> usize {
    let (pair_counts, rules) = parse(data);
    let pair_counts = iterate(pair_counts, |pair_counts| step(pair_counts, &rules))
        .nth(40)
        .unwrap();

    match pair_counts_to_elem_counts(&pair_counts)
        .into_values()
        .minmax()
    {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => panic!(),
    }
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
