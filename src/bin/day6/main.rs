use std::collections::HashMap;

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> HashMap<i8, usize> {
    data.trim().split(',').map(|v| v.parse().unwrap()).counts()
}

fn part_a(data: &'static str) -> usize {
    let mut age_counts = parse(data);
    for _ in 0..80 {
        age_counts = age_counts
            .iter()
            .map(|(&age, &count)| (age - 1, count))
            .collect();
        if let Some(births) = age_counts.remove(&-1) {
            *age_counts.entry(6).or_insert(0) += births;
            age_counts.insert(8, births);
        }
    }
    age_counts.values().sum()
}

#[allow(dead_code)]
fn part_b(data: &'static str) -> usize {
    let mut age_counts = parse(data);
    for _ in 0..256 {
        age_counts = age_counts
            .iter()
            .map(|(&age, &count)| (age - 1, count))
            .collect();
        if let Some(births) = age_counts.remove(&-1) {
            *age_counts.entry(6).or_insert(0) += births;
            age_counts.insert(8, births);
        }
    }
    age_counts.values().sum()
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
