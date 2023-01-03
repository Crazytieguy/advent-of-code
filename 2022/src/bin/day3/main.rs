#![feature(anonymous_lifetime_in_impl_trait)]
use itertools::Itertools;
use std::collections::HashSet;

const DATA: &str = include_str!("data.txt");

fn priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 'a' as usize + 1,
        'A'..='Z' => item as usize - 'A' as usize + 27,
        _ => panic!("illegal character {item}"),
    }
}

fn intersecting_item(group: impl IntoIterator<Item = &str>) -> char {
    group
        .into_iter()
        .map(|items| items.chars().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .expect("there should be more than one set")
        .into_iter()
        .exactly_one()
        .expect("there should be exactly one item in the intersection")
}

fn part_a(data: &str) -> usize {
    data.lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| intersecting_item([left, right]))
        .map(priority)
        .sum()
}

fn part_b(data: &str) -> usize {
    data.lines()
        .chunks(3)
        .into_iter()
        .map(intersecting_item)
        .map(priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 157);
        println!("part a: {}", part_a(DATA));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 70);
        println!("part b: {}", part_b(DATA));
    }
}

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}
