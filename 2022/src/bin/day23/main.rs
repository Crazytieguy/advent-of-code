use std::{
    collections::{HashMap, HashSet},
    error::Error,
    ops::RangeInclusive,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

type Parsed = HashSet<(i16, i16)>;

fn parse(data: &str) -> Parsed {
    data.lines()
        .enumerate()
        .flat_map(move |(row, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(col, _)| (row as i16, col as i16))
        })
        .collect()
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
            if directions
                .into_iter()
                .flatten()
                .all(|(drow, dcol)| !elve_positions.contains(&(drow + row, dcol + col)))
            {
                // elve doesn't need to move
                continue;
            }
            for direction in directions {
                if direction
                    .into_iter()
                    .all(|(drow, dcol)| !elve_positions.contains(&(drow + row, dcol + col)))
                {
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

fn part_a(data: &Parsed) -> usize {
    let mut elve_positions = data.clone();
    run_simulation(&mut elve_positions, 10);
    let (rows, cols) = calc_bounds(&elve_positions);
    rows.len() * cols.len() - elve_positions.len()
}

fn part_b(data: &Parsed) -> usize {
    let mut elve_positions = data.clone();
    run_simulation(&mut elve_positions, 10000).expect("not done within 10000 rounds")
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

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 110);
        println!("part a: {}", part_a(&parse(DATA)));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 20);
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
