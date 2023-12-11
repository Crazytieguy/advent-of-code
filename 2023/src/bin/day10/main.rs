#![warn(clippy::pedantic)]
use std::{borrow::Cow, collections::HashSet};

use advent_2023::{BasicSolution, Solution};
use anyhow::{anyhow, bail, Context};
use winnow::{combinator::repeat, token::any, Parser};

struct Day;

#[derive(Debug, Clone, Copy)]
enum PipeSegment {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

#[allow(clippy::enum_glob_use)]
use PipeSegment::*;

#[derive(Debug, Clone, Copy)]
enum TraversalState {
    OutsideLoop,
    InsideLoop,
    InsideTopPipe,
    InsideBottomPipe,
}

#[allow(clippy::enum_glob_use)]
use TraversalState::*;

#[derive(Debug, Clone)]
struct FieldWithLoop {
    field: Vec<Vec<PipeSegment>>,
    loop_coords: HashSet<(usize, usize)>,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");
    const SAMPLE_INPUT_B: &'static str = include_str!("sample_b.txt");

    type Common = FieldWithLoop;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 10;

    fn common(input: &'static str) -> anyhow::Result<Self::Common> {
        let mut field = input
            .lines()
            .map(|line| field_line.parse(line).map_err(anyhow::Error::msg))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let start_coords = find_and_fix_start(&mut field)?;
        let loop_coords = find_loop(&field, start_coords)?;
        Ok(FieldWithLoop { field, loop_coords })
    }

    fn part_a(field_with_loop: Cow<Self::Common>) -> anyhow::Result<Self::Answer> {
        Ok(field_with_loop.loop_coords.len() / 2)
    }

    fn part_b(FieldWithLoop { field, loop_coords }: Self::Common) -> anyhow::Result<Self::Answer> {
        count_inside_loop(&field, &loop_coords)
    }
}

fn count_inside_loop(
    data: &[Vec<PipeSegment>],
    loop_coords: &HashSet<(usize, usize)>,
) -> anyhow::Result<usize> {
    let count_line_inside_loop = |(row, line): (usize, &Vec<PipeSegment>)| {
        line.iter()
            .enumerate()
            .try_fold((0, OutsideLoop), |(count, state), (col, &segment)| {
                Ok(if loop_coords.contains(&(row, col)) {
                    let next_state = state
                        .next(segment)
                        .with_context(|| format!("Failed state transition at ({row}, {col})"))?;
                    (count, next_state)
                } else {
                    (count + usize::from(matches!(state, InsideLoop)), state)
                })
            })
            .map(|(count, _)| count)
    };
    data.iter().enumerate().map(count_line_inside_loop).sum()
}

impl TraversalState {
    /// Assumes we are traversing from west to east,
    /// and only pipe segments from the main loop are passed.
    /// Also - start is replaced by whatever pipe segment it represents
    fn next(self, segment: PipeSegment) -> anyhow::Result<Self> {
        Ok(match (self, segment) {
            (OutsideLoop, NS) | (InsideTopPipe, NW) | (InsideBottomPipe, SW) => InsideLoop,
            (OutsideLoop, NE) | (InsideLoop, SE) | (InsideBottomPipe, EW) => InsideBottomPipe,
            (OutsideLoop, SE) | (InsideLoop, NE) | (InsideTopPipe, EW) => InsideTopPipe,
            (InsideLoop, NS) | (InsideTopPipe, SW) | (InsideBottomPipe, NW) => OutsideLoop,
            (_, Ground) => bail!("Ground should not be passed to TraversalState::next"),
            (_, Start) => bail!("Start should not be passed to TraversalState::next"),
            (state, segment) => bail!("Cannot encounter {segment:?} when {state:?}"),
        })
    }
}

fn find_and_fix_start(field: &mut [Vec<PipeSegment>]) -> anyhow::Result<(usize, usize)> {
    let (start_row, start_col) = field
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(move |(col, segment)| ((row, col), segment))
        })
        .find(|(_, &segment)| matches!(segment, Start))
        .map(|(coords, _)| coords)
        .ok_or_else(|| anyhow!("No start found"))?;

    let start_segment = [NW, NE, NS, EW, SW, SE]
        .into_iter()
        .find(|&segment| {
            segment
                .adjacent_diffs()
                .into_iter()
                .flatten()
                .all(|(drow, dcol)| {
                    check_add_signed_2d((start_row, start_col), (drow, dcol))
                        .and_then(|coords| get_2d(field, coords))
                        .and_then(PipeSegment::adjacent_diffs)
                        .is_some_and(|diffs| diffs.contains(&(-drow, -dcol)))
                })
        })
        .ok_or_else(|| anyhow!("Can't identify start segment at ({start_row}, {start_col})"))?;

    field[start_row][start_col] = start_segment;
    Ok((start_row, start_col))
}

fn find_loop(
    field: &[Vec<PipeSegment>],
    start_coords: (usize, usize),
) -> anyhow::Result<HashSet<(usize, usize)>> {
    let err = |coords| anyhow!("No connected segments found at {coords:?}");
    let mut loop_coords = HashSet::from([start_coords]);
    let mut prev = start_coords;
    let mut coords = connected_segments(field, start_coords)
        .next() // select arbitrarily
        .ok_or_else(|| err(start_coords))?;
    while coords != start_coords {
        loop_coords.insert(coords);
        (prev, coords) = (
            coords,
            connected_segments(field, coords)
                .find(|&next| next != prev)
                .ok_or_else(|| err(coords))?,
        );
    }
    Ok(loop_coords)
}

fn connected_segments(
    field: &[Vec<PipeSegment>],
    from: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    get_2d(field, from)
        .and_then(PipeSegment::adjacent_diffs)
        .into_iter()
        .flatten()
        .filter_map(move |diff| check_add_signed_2d(from, diff))
}

impl PipeSegment {
    fn adjacent_diffs(self) -> Option<[(isize, isize); 2]> {
        Some(match self {
            NW => [(-1, 0), (0, -1)],
            NE => [(-1, 0), (0, 1)],
            NS => [(-1, 0), (1, 0)],
            EW => [(0, -1), (0, 1)],
            SW => [(0, -1), (1, 0)],
            SE => [(0, 1), (1, 0)],
            Ground | Start => return None,
        })
    }
}

#[allow(clippy::similar_names)]
fn check_add_signed_2d(
    (row, col): (usize, usize),
    (drow, dcol): (isize, isize),
) -> Option<(usize, usize)> {
    row.checked_add_signed(drow)
        .zip(col.checked_add_signed(dcol))
}

fn get_2d(grid: &[Vec<PipeSegment>], (row, col): (usize, usize)) -> Option<PipeSegment> {
    grid.get(row)?.get(col).copied()
}

fn field_line(input: &mut &'static str) -> winnow::PResult<Vec<PipeSegment>> {
    repeat(1.., pipe_segment).parse_next(input)
}

fn pipe_segment(input: &mut &'static str) -> winnow::PResult<PipeSegment> {
    any.verify_map(|c| match c {
        '|' => Some(NS),
        '-' => Some(EW),
        'L' => Some(NE),
        'J' => Some(NW),
        '7' => Some(SW),
        'F' => Some(SE),
        '.' => Some(Ground),
        'S' => Some(Start),
        _ => None,
    })
    .parse_next(input)
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
