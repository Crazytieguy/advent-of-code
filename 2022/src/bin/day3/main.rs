#![feature(array_chunks)]

use itertools::Itertools;
use std::collections::HashSet;

const DATA: &str = include_str!("data.txt");

type Parsed = Vec<&'static str>;

fn parse(data: &'static str) -> Parsed {
    data.lines().collect()
}

fn priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize + 1,
        'A'..='Z' => c as usize - 'A' as usize + 27,
        _ => unreachable!(),
    }
}

fn part_a(data: &Parsed) -> usize {
    data.iter()
        .map(|line| {
            let (left, right) = (&line[..line.len() / 2], &line[line.len() / 2..]);
            let intersection = left
                .chars()
                .collect::<HashSet<_>>()
                .intersection(&right.chars().collect())
                .copied()
                .exactly_one()
                .unwrap();
            priority(intersection)
        })
        .sum()
}

fn part_b(data: &Parsed) -> usize {
    data.array_chunks::<3>()
        .map(|[first, second, thid]| {
            let mut intersection = first.chars().collect::<HashSet<_>>();
            intersection = intersection
                .intersection(&second.chars().collect())
                .copied()
                .collect();
            let l = intersection
                .intersection(&thid.chars().collect())
                .copied()
                .exactly_one()
                .unwrap();
            priority(l)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 157);
        println!("part a: {}", part_a(&parse(DATA)));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 70);
        println!("part b: {}", part_b(&parse(DATA)));
    }
}

fn main() {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
}
