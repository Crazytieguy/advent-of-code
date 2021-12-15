use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    iter,
};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> HashMap<(i16, i16), u32> {
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            iter::repeat(y)
                .zip(line.chars().enumerate())
                .map(|(y, (x, c))| ((x as i16, y as i16), c.to_digit(10).unwrap()))
        })
        .collect()
}

fn part_a(data: &'static str) -> u32 {
    let grid = parse(data);
    best_total_risk(&grid)
}

fn best_total_risk(grid: &HashMap<(i16, i16), u32>) -> u32 {
    let mut best_known = HashMap::new();
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0)]);
    while let Some((Reverse(total_risk), x, y)) = queue.pop() {
        let best_known_risk = best_known.entry((x, y)).or_insert(u32::MAX);
        if total_risk < *best_known_risk {
            *best_known_risk = total_risk;
            for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let (x, y) = (x + dx, y + dy);
                if let Some(risk) = grid.get(&(x, y)) {
                    queue.push((Reverse(total_risk + risk), x, y));
                }
            }
        }
    }
    best_known[best_known.keys().max().unwrap()]
}

fn part_b(data: &'static str) -> u32 {
    let grid = parse(data);
    let (tile_width, tile_height) = grid.keys().max().map(|(x, y)| (x + 1, y + 1)).unwrap();
    let grid = grid
        .into_iter()
        .flat_map(|((x, y), risk)| {
            (0..5)
                .cartesian_product(0..5)
                .map(|(tile_x, tile_y)| {
                    (
                        (tile_x * tile_width + x, tile_y * tile_height + y),
                        (risk + tile_x as u32 + tile_y as u32 - 1) % 9 + 1,
                    )
                })
                .collect_vec()
        })
        .collect();
    best_total_risk(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 40);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 315);
    }
}
