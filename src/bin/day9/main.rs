use std::collections::HashSet;

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
    (row, col): (usize, usize),
    num_rows: usize,
    num_cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        row.checked_sub(1).zip(Some(col)),                          // up
        Some(row).zip(col.checked_sub(1)),                          // left
        Some(row + 1).filter(|&row| row < num_rows).zip(Some(col)), // down
        Some(row).zip(Some(col + 1).filter(|&col| col < num_cols)), // right
    ]
    .into_iter()
    .flatten()
}

fn get_low_points(data: &[Vec<usize>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    (0..data.len())
        .cartesian_product(0..data[0].len())
        .filter(|&point| {
            adjacent_coords(point, data.len(), data[0].len())
                .all(|adj| data[adj.0][adj.1] > data[point.0][point.1])
        })
}

fn part_a(data: &'static str) -> usize {
    let data = parse(data);
    get_low_points(&data)
        .map(|(row, col)| data[row][col] + 1)
        .sum()
}

fn get_basin_size(coords: (usize, usize), data: &[Vec<usize>]) -> usize {
    let mut basin = HashSet::new();
    crawl(coords, data, &mut basin);
    basin.len()
}

fn crawl(coords: (usize, usize), data: &[Vec<usize>], basin: &mut HashSet<(usize, usize)>) {
    if basin.contains(&coords) || data[coords.0][coords.1] == 9 {
        return;
    }
    basin.insert(coords);
    adjacent_coords(coords, data.len(), data[0].len()).for_each(|coords| crawl(coords, data, basin))
}

fn part_b(data: &'static str) -> usize {
    let data = parse(data);
    get_low_points(&data)
        .map(|coords| get_basin_size(coords, &data))
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
