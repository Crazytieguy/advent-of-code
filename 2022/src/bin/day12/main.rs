use std::{collections::VecDeque, error::Error};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

type Position = (usize, usize);
type HeightMap<'a> = Vec<&'a [u8]>;
type Parsed<'a> = (Position, Position, HeightMap<'a>);

fn parse(data: &str) -> Parsed {
    let height_map = data.lines().map(|row| row.as_bytes()).collect_vec();
    let iter_positions = || (0..height_map.len()).cartesian_product(0..height_map[0].len());
    let start = iter_positions()
        .find(|&(x, y)| height_map[x][y] == b'S')
        .expect("There should be an 'S' character in the input");
    let end = iter_positions()
        .find(|&(x, y)| height_map[x][y] == b'E')
        .expect("There should be an 'E' character in the input");
    (start, end, height_map)
}

fn normalize_height(mark: u8) -> u8 {
    match mark {
        b'S' => b'a',
        b'E' => b'z',
        h => h,
    }
}

fn reverse_bfs(
    best_reception: Position,
    height_map: &HeightMap,
    stop_condition: impl Fn(Position) -> bool,
) -> usize {
    let (num_rows, num_columns) = (height_map.len(), height_map[0].len());
    let height = |(x, y): Position| normalize_height(height_map[x][y]);
    let mut seen = vec![vec![false; num_columns]; num_rows];
    let mut queue = VecDeque::from([(0, best_reception)]);

    while let Some((steps, (x, y))) = queue.pop_front() {
        if seen[x][y] {
            continue;
        }
        if stop_condition((x, y)) {
            return steps;
        }
        seen[x][y] = true;

        let checked_2d_diff = |(dx, dy)| {
            let x = x.checked_add_signed(dx).filter(|&x| x < num_rows);
            let y = y.checked_add_signed(dy).filter(|&y| y < num_columns);
            x.zip(y)
        };

        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(checked_2d_diff)
            .filter(|&neighbor| height(neighbor) >= height((x, y)) - 1)
            .for_each(|neighbor| queue.push_back((steps + 1, neighbor)));
    }
    unreachable!("no path found")
}

fn part_a((start, end, height_map): &Parsed) -> usize {
    reverse_bfs(*end, height_map, |position| position == *start)
}

fn part_b((_start, end, height_map): &Parsed) -> usize {
    reverse_bfs(*end, height_map, |(x, y)| height_map[x][y] == b'a')
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 31);
        println!("part a: {}", part_a(&parse(DATA)));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 29);
        println!("part b: {}", part_b(&parse(DATA)));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
