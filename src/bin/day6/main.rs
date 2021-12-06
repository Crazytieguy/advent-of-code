use std::collections::HashMap;

use itertools::{iterate, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> HashMap<i8, usize> {
    data.trim().split(',').map(|v| v.parse().unwrap()).counts()
}

fn next_state(age_counts: &HashMap<i8, usize>) -> HashMap<i8, usize> {
    let mut age_counts: HashMap<_, _> = age_counts
        .iter()
        .map(|(&age, &count)| (age - 1, count))
        .collect();
    if let Some(births) = age_counts.remove(&-1) {
        *age_counts.entry(6).or_insert(0) += births;
        age_counts.insert(8, births);
    }
    age_counts
}

fn part_a(data: &'static str) -> usize {
    iterate(parse(data), next_state)
        .nth(80)
        .unwrap()
        .values()
        .sum()
}

fn part_b(data: &'static str) -> usize {
    iterate(parse(data), next_state)
        .nth(256)
        .unwrap()
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 5934);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 26984457539);
    }
}
