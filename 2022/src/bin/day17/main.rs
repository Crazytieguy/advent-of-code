#![feature(array_windows)]

use itertools::Itertools;
use nom::{branch::alt, character::complete::char, multi::many1};
use nom_supreme::ParserExt;
use std::error::Error;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
}

type Parsed = Vec<Direction>;

fn parse(data: &str) -> IResult<Parsed> {
    many1(alt((
        char('>').value(Direction::Right),
        char('<').value(Direction::Left),
    )))(data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    NA,
    AA,
    BB,
    CC,
    DD,
    EE,
}

use Cell::*;

fn gen_rocks() -> impl Iterator<Item = Vec<[Cell; 7]>> {
    let rocks = vec![
        vec![
            // ####
            [NA, NA, AA, AA, AA, AA, NA],
        ],
        vec![
            // .#.
            // ###
            // .#.
            [NA, NA, NA, BB, NA, NA, NA],
            [NA, NA, BB, BB, BB, NA, NA],
            [NA, NA, NA, BB, NA, NA, NA],
        ],
        vec![
            // ..#
            // ..#
            // ###
            [NA, NA, NA, NA, CC, NA, NA],
            [NA, NA, NA, NA, CC, NA, NA],
            [NA, NA, CC, CC, CC, NA, NA],
        ],
        vec![
            // #
            // #
            // #
            // #
            [NA, NA, DD, NA, NA, NA, NA],
            [NA, NA, DD, NA, NA, NA, NA],
            [NA, NA, DD, NA, NA, NA, NA],
            [NA, NA, DD, NA, NA, NA, NA],
        ],
        vec![
            // ##
            // ##
            [NA, NA, EE, EE, NA, NA, NA],
            [NA, NA, EE, EE, NA, NA, NA],
        ],
    ];
    rocks.into_iter().cycle()
}

fn simulate(air_directions: &Parsed, num_rocks: usize) -> Vec<[Cell; 7]> {
    let mut chamber: Vec<[Cell; 7]> = vec![];
    let mut air_directions = air_directions.iter().cycle();
    for mut rock in gen_rocks().take(num_rocks) {
        let mut rock_top = chamber.len() + 3 + rock.len();
        loop {
            rock_top -= 1;
            let &dir = air_directions.next().unwrap();
            let air_push_successful = rock
                .iter()
                .enumerate()
                .map(|(height, row)| (row, chamber.get(rock_top - height)))
                .all(|(row, chamber_row)| {
                    if match dir {
                        Direction::Right => !matches!(row[6], NA),
                        Direction::Left => !matches!(row[0], NA),
                    } {
                        return false;
                    }
                    let Some(chamber_row) = chamber_row else {
                        return true;
                    };
                    match dir {
                        Direction::Right => row[0..6]
                            .iter()
                            .zip(chamber_row[1..7].iter())
                            .all(|(a, b)| matches!(a, NA) || matches!(b, NA)),
                        Direction::Left => row[1..7]
                            .iter()
                            .zip(chamber_row[0..6].iter())
                            .all(|(a, b)| matches!(a, NA) || matches!(b, NA)),
                    }
                });
            if air_push_successful {
                rock.iter_mut().for_each(|row| {
                    row.rotate_right(match dir {
                        Direction::Right => 1,
                        Direction::Left => 6,
                    });
                });
            }
            if rock_top < rock.len() {
                break;
            }
            let move_down_successful = rock.iter().enumerate().all(|(row_idx, row)| {
                if let Some(chamber_row) = chamber.get(rock_top - row_idx - 1) {
                    row.iter()
                        .zip(chamber_row.iter())
                        .all(|(a, b)| matches!(a, NA) || matches!(b, NA))
                } else {
                    true
                }
            });
            if !move_down_successful {
                break;
            }
        }
        chamber.resize(chamber.len().max(rock_top + 1), [NA; 7]);
        for (row_idx, row) in rock.into_iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if !matches!(cell, NA) {
                    chamber[rock_top - row_idx][x] = *cell;
                }
            }
        }
    }
    chamber
}

fn part_a(data: &Parsed) -> usize {
    simulate(data, 2022).len()
}

fn count_rocks_in_chamber_slice(slice: &[[Cell; 7]]) -> usize {
    slice
        .iter()
        .flatten()
        .filter(|&&cell| !matches!(cell, NA))
        .count()
        * 5
        / 22
}

fn part_b(data: &Parsed) -> u64 {
    let chamber = simulate(data, 5000);
    let (pattern_start, pattern_length) = chamber
        .windows(50)
        .enumerate()
        .tuple_combinations()
        .find(|((_, a), (_, b))| a == b)
        .map(|((i, _), (j, _))| (i, j - i))
        .expect("There should be a pattern!");

    let rocks_before_pattern = count_rocks_in_chamber_slice(&chamber[..pattern_start]);

    let rocks_generated_in_pattern =
        count_rocks_in_chamber_slice(&chamber[pattern_start..pattern_start + pattern_length]);
    let num_pattern_repetitions =
        (1_000_000_000_000 - rocks_before_pattern as u64) / rocks_generated_in_pattern as u64;
    let leftover_rocks =
        (1_000_000_000_000 - rocks_before_pattern as u64) % rocks_generated_in_pattern as u64;
    let leftover_rocks_height = (0..=pattern_length)
        .find(|&i| {
            count_rocks_in_chamber_slice(&chamber[pattern_start..pattern_start + i])
                >= leftover_rocks as usize
        })
        .expect("There should be a leftover rock height");
    // print_chamber(&chamber);
    num_pattern_repetitions * pattern_length as u64
        + pattern_start as u64
        + leftover_rocks_height as u64
}

// Was really useful for debugging
#[allow(dead_code)]
fn print_chamber(chamber: &[[Cell; 7]]) -> Vec<u8> {
    let mut buf = Vec::new();
    for row in chamber.iter().rev() {
        for cell in row {
            buf.push(match cell {
                NA => b'.',
                AA => b'#',
                BB => b'O',
                CC => b'X',
                DD => b'@',
                EE => b'*',
            });
        }
        buf.push(b'\n');
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 3068);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 1_514_285_714_288);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
