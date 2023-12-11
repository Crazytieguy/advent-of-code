#![warn(clippy::pedantic)]
use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;
use itertools::Itertools;

struct Day;

const NORTH: (isize, isize) = (-1, 0);
const SOUTH: (isize, isize) = (1, 0);
const WEST: (isize, isize) = (0, -1);
const EAST: (isize, isize) = (0, 1);

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");
    const SAMPLE_INPUT_B: &'static str = include_str!("sample_b.txt");

    type Shared = Vec<(usize, usize)>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 10;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        let field = input.lines().map(str::as_bytes).collect_vec();
        let start_coords = find_start_coords(&field)?;
        find_loop(&field, start_coords)
    }

    fn part_a(loop_coords: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(loop_coords.len() / 2)
    }

    fn part_b(loop_coords: Self::Shared) -> anyhow::Result<Self::Answer> {
        let area = shoelace_formula(&loop_coords)?;
        // Pick's theorem: A = i + b/2 - 1
        Ok(area + 1 - loop_coords.len() / 2)
    }
}

fn shoelace_formula(loop_coords: &[(usize, usize)]) -> Result<usize, anyhow::Error> {
    Ok(loop_coords
        .iter()
        .copied()
        .chain([loop_coords[0]])
        .tuple_windows()
        .map(|((y1, x1), (y2, x2))| Ok(isize::try_from(x1 * y2)? - isize::try_from(x2 * y1)?))
        .sum::<anyhow::Result<isize>>()?
        .unsigned_abs()
        / 2)
}

fn find_loop(
    field: &[&[u8]],
    start_coords: (usize, usize),
) -> Result<Vec<(usize, usize)>, anyhow::Error> {
    let err = |coords| anyhow!("No connected segments found at {coords:?}");
    let mut loop_coords = vec![start_coords];
    let mut prev = start_coords;
    let mut coords = [NORTH, SOUTH, WEST, EAST]
        .into_iter()
        .filter_map(|diff| check_add_signed_2d(start_coords, diff))
        .find(|&coords| connected_segments(field, coords).contains(&start_coords))
        .ok_or_else(|| err(start_coords))?;

    while coords != start_coords {
        loop_coords.push(coords);
        (prev, coords) = (
            coords,
            connected_segments(field, coords)
                .find(|&next| next != prev)
                .ok_or_else(|| err(coords))?,
        );
    }
    Ok(loop_coords)
}

fn find_start_coords(field: &[&[u8]]) -> anyhow::Result<(usize, usize)> {
    field
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, segment)| ((row, col), segment))
        })
        .find(|(_, &segment)| segment == b'S')
        .map(|(coords, _)| coords)
        .ok_or_else(|| anyhow!("No start found"))
}

fn connected_segments(
    field: &[&[u8]],
    from: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    get_2d(field, from)
        .and_then(adjacent_diffs)
        .into_iter()
        .flatten()
        .filter_map(move |diff| check_add_signed_2d(from, diff))
}

fn adjacent_diffs(pipe_segment: u8) -> Option<[(isize, isize); 2]> {
    Some(match pipe_segment {
        b'J' => [NORTH, WEST],
        b'L' => [NORTH, EAST],
        b'|' => [NORTH, SOUTH],
        b'-' => [WEST, EAST],
        b'7' => [WEST, SOUTH],
        b'F' => [EAST, SOUTH],
        _ => return None,
    })
}

#[allow(clippy::similar_names)]
fn check_add_signed_2d(
    (row, col): (usize, usize),
    (drow, dcol): (isize, isize),
) -> Option<(usize, usize)> {
    row.checked_add_signed(drow)
        .zip(col.checked_add_signed(dcol))
}

fn get_2d(field: &[&[u8]], (row, col): (usize, usize)) -> Option<u8> {
    field.get(row)?.get(col).copied()
}

fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_part_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_part_b()
    }
}
