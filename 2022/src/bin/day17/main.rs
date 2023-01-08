#![feature(array_windows)]
use advent_2022::*;
use itertools::Itertools;
use nom::{branch::alt, character::complete::char, multi::many1};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<Direction>;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 3068;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 1_514_285_714_288;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        many1(alt((
            char('>').value(Direction::Right),
            char('<').value(Direction::Left),
        )))(data)
    }

    fn a(data: Self::Parsed) -> usize {
        simulate(&data, 2022).len()
    }

    fn b(data: Self::Parsed) -> usize {
        let chamber = simulate(&data, 5000);
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
            (1_000_000_000_000 - rocks_before_pattern) / rocks_generated_in_pattern;
        let leftover_rocks =
            (1_000_000_000_000 - rocks_before_pattern) % rocks_generated_in_pattern;
        let leftover_rocks_height = (0..=pattern_length)
            .find(|&i| {
                count_rocks_in_chamber_slice(&chamber[pattern_start..pattern_start + i])
                    >= leftover_rocks
            })
            .expect("There should be a leftover rock height");
        // print_chamber(&chamber);
        num_pattern_repetitions * pattern_length + pattern_start + leftover_rocks_height
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
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

fn simulate(air_directions: &[Direction], num_rocks: usize) -> Vec<[Cell; 7]> {
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

fn count_rocks_in_chamber_slice(slice: &[[Cell; 7]]) -> usize {
    slice
        .iter()
        .flatten()
        .filter(|&&cell| !matches!(cell, NA))
        .count()
        * 5
        / 22
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
