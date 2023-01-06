use std::{collections::VecDeque, error::Error};

use itertools::izip;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

#[derive(Debug, Default, Clone)]
struct Blizzards {
    up: VecDeque<u128>,
    down: VecDeque<u128>,
    left: Vec<u128>,
    right: Vec<u128>,
}

fn mask(width: usize) -> u128 {
    (1 << width) - 1
}

impl Blizzards {
    fn update(&mut self, width: usize) {
        self.up.rotate_left(1);
        self.down.rotate_right(1);
        self.left.iter_mut().for_each(|row| {
            *row = (*row >> 1) | ((*row & 1) << (width - 1));
        });
        self.right.iter_mut().for_each(|row| {
            *row = (*row << 1) | (*row >> (width - 1));
            *row &= mask(width);
        });
    }
}

fn parse(data: &str) -> (Blizzards, usize) {
    let width = data.find('\n').expect("no newline") - 2;
    let (up, (down, (left, right))) = data
        .lines()
        .filter(|line| &line[2..3] != "#")
        .map(|line| {
            let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
            line.bytes()
                .filter(|&c| c != b'#')
                .enumerate()
                .for_each(|(col, c)| {
                    let bit = 1 << col;
                    match c {
                        b'>' => right |= bit,
                        b'<' => left |= bit,
                        b'^' => up |= bit,
                        b'v' => down |= bit,
                        _ => {}
                    };
                });
            (up, (down, (left, right)))
        })
        .unzip();
    (
        Blizzards {
            up,
            down,
            left,
            right,
        },
        width,
    )
}

fn adjacent_positions<const HEIGHT: usize>(positions: &[u128], width: usize) -> [u128; HEIGHT] {
    let mut new_positions = [0; HEIGHT];
    for (row, above, cur, bellow) in izip!(
        &mut new_positions,
        [0].iter().chain(positions),
        positions,
        positions.iter().skip(1).chain([0].iter())
    ) {
        *row = (cur | cur << 1 | cur >> 1 | above | bellow) & mask(width);
    }
    new_positions
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Exit,
    Entrance,
}

use Destination::*;

fn simulate_shortest_path<const HEIGHT: usize>(
    blizzards: &mut Blizzards,
    width: usize,
    destination: Destination,
) -> usize {
    assert_eq!(HEIGHT, blizzards.right.len());
    let mut positions = [0; HEIGHT];
    for minute in 1.. {
        blizzards.update(width);
        positions = adjacent_positions(&positions, width);
        match destination {
            Exit => positions[0] |= 1,
            Entrance => positions[HEIGHT - 1] |= 1 << (width - 1),
        }
        for (p, up, down, left, right) in izip!(
            &mut positions,
            &blizzards.up,
            &blizzards.down,
            &blizzards.left,
            &blizzards.right
        ) {
            *p &= !(up | down | left | right);
        }
        if matches!(destination, Exit) && positions[HEIGHT - 1] >> (width - 1) == 1
            || matches!(destination, Entrance) && positions[0] & 1 == 1
        {
            blizzards.update(width);
            return minute + 1;
        }
    }
    unreachable!()
}

fn part_a<const HEIGHT: usize>(blizzards: &mut Blizzards, width: usize) -> usize {
    simulate_shortest_path::<HEIGHT>(blizzards, width, Exit)
}

fn part_b<const HEIGHT: usize>(
    blizzards: &mut Blizzards,
    width: usize,
    part_a_ans: usize,
) -> usize {
    part_a_ans
        + simulate_shortest_path::<HEIGHT>(blizzards, width, Entrance)
        + simulate_shortest_path::<HEIGHT>(blizzards, width, Exit)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        let (mut blizzards, width) = parse(SAMPLE_DATA);
        assert_eq!(part_a::<4>(&mut blizzards, width), 18);
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        let (mut blizzards, width) = parse(SAMPLE_DATA);
        let a = part_a::<4>(&mut blizzards, width);
        assert_eq!(part_b::<4>(&mut blizzards, width, a), 54);
        Ok(())
    }
}

fn main() -> OutResult {
    let (mut blizzards, width) = parse(DATA);
    let start = std::time::Instant::now();
    let a = part_a::<25>(&mut blizzards, width);
    println!("part a: {a}");
    let b = part_b::<25>(&mut blizzards, width, a);
    println!("part b: {b}");
    println!("{:?}", start.elapsed());
    Ok(())
}
