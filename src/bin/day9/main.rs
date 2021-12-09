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

fn adjacent_coords(
    row: usize,
    col: usize,
    num_rows: usize,
    num_cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        row.checked_sub(1).zip(Some(col)),
        Some(row).zip(col.checked_sub(1)),
        Some(row + 1).filter(|&row| row < num_rows).zip(Some(col)),
        Some(row).zip(Some(col + 1).filter(|&col| col < num_cols)),
    ]
    .into_iter()
    .flatten()
}

fn part_a(data: &'static str) -> usize {
    let data = parse(data);
    (0..data.len())
        .cartesian_product(0..data[0].len())
        .filter(|&(row, col)| {
            adjacent_coords(row, col, data.len(), data[0].len())
                .all(|(adj_row, adj_col)| data[adj_row][adj_col] > data[row][col])
        })
        .map(|(row, col)| data[row][col] + 1)
        .sum()
}

fn crawl(row: usize, col: usize, data: &[Vec<usize>], basin: &mut HashSet<(usize, usize)>) {
    if basin.contains(&(row, col)) || data[row][col] == 9 {
        return;
    }
    basin.insert((row, col));
    adjacent_coords(row, col, data.len(), data[0].len())
        .for_each(|(row, col)| crawl(row, col, data, basin))
}

fn part_b(data: &'static str) -> usize {
    let data = parse(data);
    let mut coords_to_basin = HashMap::new();
    for row in 0..data.len() {
        for col in 0..data[0].len() {
            if coords_to_basin.contains_key(&(row, col)) {
                continue;
            }
            let mut basin = HashSet::new();
            crawl(row, col, &data, &mut basin);
            coords_to_basin.extend(basin.into_iter().zip(repeat((row, col))));
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
