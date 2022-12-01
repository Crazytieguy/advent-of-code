use std::cmp::Reverse;

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn parse(data: &'static str) -> Vec<Vec<usize>> {
    data.split("\n\n")
        .map(|inventory| inventory.lines().map(|l| l.parse().unwrap()).collect())
        .collect()
}

fn part_a(data: &'static str) -> usize {
    let inventories = parse(data);
    inventories
        .into_iter()
        .map(|inventory| inventory.into_iter().sum())
        .max()
        .unwrap()
}

fn part_b(data: &'static str) -> usize {
    let inventories = parse(data);
    inventories
        .into_iter()
        .map(|inventory| inventory.into_iter().sum::<usize>())
        .sorted_unstable_by_key(|&cals| Reverse(cals))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 24000);
        println!("part a: {}", part_a(DATA));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 45000);
        println!("part b: {}", part_b(DATA));
    }
}

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}
