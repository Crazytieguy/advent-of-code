use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> HashMap<(i64, i64), usize> {
    data.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .map(move |(col, val)| ((row as i64, col as i64), val))
        })
        .collect()
}

fn adjacent_points(
    (y, x): (i64, i64),
    data: &HashMap<(i64, i64), usize>,
) -> impl Iterator<Item = (i64, i64)> + '_ {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .map(|(dy, dx)| (y + dy, x + dx))
        .into_iter()
        .filter(|p| data.contains_key(p))
}

fn get_low_points(data: &HashMap<(i64, i64), usize>) -> impl Iterator<Item = (i64, i64)> + '_ {
    data.keys()
        .copied()
        .filter(|&p| adjacent_points(p, data).all(|adj| data[&adj] > data[&p]))
}

fn part_a(data: &'static str) -> usize {
    let data = parse(data);
    get_low_points(&data).map(|p| data[&p] + 1).sum()
}

fn get_basin_size(p: (i64, i64), data: &HashMap<(i64, i64), usize>) -> usize {
    let mut basin = HashSet::new();
    crawl(p, data, &mut basin);
    basin.len()
}

fn crawl(p: (i64, i64), data: &HashMap<(i64, i64), usize>, basin: &mut HashSet<(i64, i64)>) {
    if basin.contains(&p) || data[&p] == 9 {
        return;
    }
    basin.insert(p);
    adjacent_points(p, data).for_each(|p| crawl(p, data, basin))
}

fn part_b(data: &'static str) -> usize {
    let data = parse(data);
    get_low_points(&data)
        .map(|p| get_basin_size(p, &data))
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 15);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 1134);
    }
}
