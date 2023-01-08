use advent_2022::*;
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = HashSet<(i16, i16)>;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 110;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 20;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        Ok((
            "",
            data.lines()
                .enumerate()
                .flat_map(move |(row, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|&(_, c)| c == '#')
                        .map(move |(col, _)| (row as i16, col as i16))
                })
                .collect(),
        ))
    }

    fn a(mut elve_positions: Self::Parsed) -> Self::Answer {
        run_simulation(&mut elve_positions, 10);
        let (rows, cols) = calc_bounds(&elve_positions);
        rows.len() * cols.len() - elve_positions.len()
    }

    fn b(mut elve_positions: Self::Parsed) -> Self::Answer {
        run_simulation(&mut elve_positions, 10000).expect("not done within 10000 rounds")
    }
}

fn run_simulation(elve_positions: &mut HashSet<(i16, i16)>, max_rounds: usize) -> Option<usize> {
    let mut directions = [
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
    ];
    let mut propositions = HashMap::new();
    for round in 1..=max_rounds {
        for &(row, col) in elve_positions.iter() {
            let is_available = |(drow, dcol)| !elve_positions.contains(&(drow + row, dcol + col));
            if directions.into_iter().flatten().all(is_available) {
                // elve doesn't need to move
                continue;
            }
            for direction in directions {
                if direction.into_iter().all(is_available) {
                    // elve chooses this direction
                    let (drow, dcol) = direction[1];
                    let key = (row + drow, col + dcol);
                    if propositions.remove(&key).is_none() {
                        propositions.insert(key, (row, col));
                    }
                    break;
                }
            }
        }
        if propositions.is_empty() {
            return Some(round);
        }
        for (to, from) in propositions.drain() {
            elve_positions.remove(&from);
            elve_positions.insert(to);
        }
        directions.rotate_left(1);
    }
    None
}

fn calc_bounds(elve_positions: &HashSet<(i16, i16)>) -> (RangeInclusive<i16>, RangeInclusive<i16>) {
    let mut min_row = i16::MAX;
    let mut max_row = i16::MIN;
    let mut min_col = i16::MAX;
    let mut max_col = i16::MIN;
    for &(row, col) in elve_positions {
        min_row = min_row.min(row);
        max_row = max_row.max(row);
        min_col = min_col.min(col);
        max_col = max_col.max(col);
    }
    (min_row..=max_row, min_col..=max_col)
}

// for debugging
#[allow(dead_code)]
fn print_elve_positions(elves: &HashSet<(i16, i16)>) {
    let (rows, cols) = calc_bounds(elves);
    for row in rows {
        for col in cols.clone() {
            if elves.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
