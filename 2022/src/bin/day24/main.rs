use std::error::Error;

use bitvec::prelude::BitArray;
use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

type BitGrid = [BitArray<[u64; 2]>; 25];

#[derive(Debug, Default, Clone, Copy)]
struct Blizzards {
    up: BitGrid,
    down: BitGrid,
    left: BitGrid,
    right: BitGrid,
}

impl Blizzards {
    fn update(&mut self, height: usize, width: usize) {
        self.up[..height].rotate_left(1);
        self.down[..height].rotate_right(1);
        self.left
            .iter_mut()
            .for_each(|row| row[..width].rotate_left(1));
        self.right
            .iter_mut()
            .for_each(|row| row[..width].rotate_right(1));
    }
}

fn parse(data: &str) -> (Blizzards, usize, usize) {
    let lines = data.lines().collect_vec();
    let height = lines.len() - 2;
    let width = lines[0].len() - 2;
    let mut blizzards = Blizzards::default();
    lines
        .into_iter()
        .filter(|line| &line[2..3] != "#")
        .enumerate()
        .for_each(|(row, line)| {
            line.chars()
                .filter(|&c| c != '#')
                .enumerate()
                .for_each(|(col, c)| {
                    match c {
                        '>' => blizzards.right[row].set(col, true),
                        '<' => blizzards.left[row].set(col, true),
                        '^' => blizzards.up[row].set(col, true),
                        'v' => blizzards.down[row].set(col, true),
                        _ => {}
                    };
                })
        });
    (blizzards, height, width)
}

fn adjacent_positions(positions: &BitGrid, height: usize, width: usize) -> BitGrid {
    let mut next_positions = *positions;
    for row in 0..height {
        next_positions[row][1..width] |= &positions[row][0..width - 1];
        next_positions[row][0..width - 1] |= &positions[row][1..width];
        if row > 0 {
            next_positions[row] |= &positions[row - 1];
        }
        if row < height - 1 {
            next_positions[row] |= &positions[row + 1];
        }
    }
    next_positions
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Exit,
    Entrance,
}

use Destination::*;

#[allow(clippy::needless_range_loop)]
fn simulate_shortest_path(
    blizzards: &mut Blizzards,
    height: usize,
    width: usize,
    destination: Destination,
) -> usize {
    let mut positions = BitGrid::default();
    for minute in 1.. {
        blizzards.update(height, width);
        positions = adjacent_positions(&positions, height, width);
        match destination {
            Exit => positions[0].set(0, true),
            Entrance => positions[height - 1].set(width - 1, true),
        }
        for row in 0..height {
            positions[row] &= &!(blizzards.up[row]
                | blizzards.down[row]
                | blizzards.left[row]
                | blizzards.right[row]);
        }
        if matches!(destination, Exit) && positions[height - 1][width - 1]
            || matches!(destination, Entrance) && positions[0][0]
        {
            blizzards.update(height, width);
            return minute + 1;
        }
    }
    unreachable!()
}

fn part_a((mut blizzards, height, width): (Blizzards, usize, usize)) -> usize {
    simulate_shortest_path(&mut blizzards, height, width, Exit)
}

fn part_b((mut blizzards, height, width): (Blizzards, usize, usize)) -> usize {
    simulate_shortest_path(&mut blizzards, height, width, Exit)
        + simulate_shortest_path(&mut blizzards, height, width, Entrance)
        + simulate_shortest_path(&mut blizzards, height, width, Exit)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(parse(SAMPLE_DATA)), 18);
        println!("part a: {}", part_a(parse(DATA)));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(parse(SAMPLE_DATA)), 54);
        println!("part b: {}", part_b(parse(DATA)));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(parsed));
    println!("part b: {}", part_b(parsed));
    Ok(())
}
