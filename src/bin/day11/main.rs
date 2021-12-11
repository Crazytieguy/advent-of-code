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
                .map(|(col, e)| ((row as i32, col as i32), e))
                .collect_vec()
        })
        .collect()
}

fn flash_if_gt_9(grid: &mut HashMap<(i32, i32), u32>, (y, x): (i32, i32)) {
    let e = match grid.get_mut(&(y, x)) {
        Some(e) => e,
        None => return,
    };
    if *e <= 9 {
        return;
    }
    *e = 0;
    [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ]
    .map(|(dy, dx)| (y + dy, x + dx))
    .into_iter()
    .for_each(|adj| {
        if let Some(e_adj) = grid.get_mut(&adj) {
            if *e_adj > 0 {
                *e_adj += 1;
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
    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut grid);
    }
    sum
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
