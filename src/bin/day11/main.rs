use std::{collections::HashMap, iter::repeat_with};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> HashMap<(i32, i32), u32> {
    data.lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .map(|(col, energy)| ((row as i32, col as i32), energy))
                .collect_vec()
        })
        .collect()
}

fn flash_if_gt_9(grid: &mut HashMap<(i32, i32), u32>, (y, x): (i32, i32)) {
    match grid.get_mut(&(y, x)) {
        Some(energy) if *energy >= 9 => *energy = 0,
        _ => return,
    };
    (-1..=1)
        .cartesian_product(-1..=1)
        .map(|(dy, dx)| (y + dy, x + dx))
        .for_each(|adj| {
            if let Some(energy) = grid.get_mut(&adj) {
                if *energy > 0 {
                    *energy += 1;
                    flash_if_gt_9(grid, adj)
                }
            }
        })
}

fn step(grid: &mut HashMap<(i32, i32), u32>) -> usize {
    grid.values_mut().for_each(|e| *e += 1);
    for p in (0..10).cartesian_product(0..10) {
        flash_if_gt_9(grid, p);
    }
    grid.values().filter(|&&e| e == 0).count()
}

fn part_a(data: &'static str) -> usize {
    let mut grid = parse(data);
    repeat_with(|| step(&mut grid)).take(100).sum()
}

fn part_b(data: &'static str) -> usize {
    let mut grid = parse(data);
    repeat_with(|| step(&mut grid))
        .find_position(|&flashes| flashes == 100)
        .map(|(pos, _)| pos)
        .unwrap()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 1656);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 195);
    }
}
