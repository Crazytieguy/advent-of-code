use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> Vec<Vec<usize>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn part_a(data: &'static str) -> usize {
    let data = parse(data);
    let mut sum = 0;
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            let num = data[row][col];
            let is_low = [
                row.checked_sub(1).zip(Some(col)),
                Some(row).zip(col.checked_sub(1)),
                if col + 1 < data[0].len() {
                    Some((row, col + 1))
                } else {
                    None
                },
                if row + 1 < data.len() {
                    Some((row + 1, col))
                } else {
                    None
                },
            ]
            .into_iter()
            .flatten()
            .all(|(row, col)| data[row][col] > num);
            if is_low {
                sum += num + 1;
            }
        }
    }
    sum
}

fn crawl(row: usize, col: usize, data: &[Vec<usize>], basin: &mut HashSet<(usize, usize)>) {
    if basin.contains(&(row, col))
        || row >= data.len()
        || col >= data[0].len()
        || data[row][col] == 9
    {
        return;
    }
    basin.insert((row, col));
    if row > 0 {
        crawl(row - 1, col, data, basin);
    }
    if col > 0 {
        crawl(row, col - 1, data, basin)
    }
    crawl(row + 1, col, data, basin);
    crawl(row, col + 1, data, basin);
}

fn part_b(data: &'static str) -> usize {
    let data = parse(data);
    let mut basin_id = 0;
    let mut coords_to_basin = HashMap::new();
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if coords_to_basin.contains_key(&(row, col)) {
                continue;
            }
            let mut basin = HashSet::new();
            crawl(row, col, &data, &mut basin);
            if !basin.is_empty() {
                coords_to_basin.extend(basin.into_iter().zip(repeat(basin_id)));
                basin_id += 1;
            }
        }
    }
    coords_to_basin
        .into_values()
        .counts()
        .into_values()
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
