use std::collections::HashSet;

use advent_2023::{BasicSolution, Solution};
use anyhow::{anyhow, bail, Context};
use winnow::{combinator::repeat, token::any, Parser};

struct Day;

#[derive(Debug, Clone, Copy)]
enum PipeSegment {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

use PipeSegment::*;

#[derive(Debug, Clone, Copy)]
enum TraversalState {
    OutsideLoop,
    InsideLoop,
    InsideTopPipe,
    InsideBottomPipe,
}

use TraversalState::*;

#[derive(Debug, Clone)]
struct FieldWithLoop {
    field: Vec<Vec<PipeSegment>>,
    loop_coords: HashSet<(usize, usize)>,
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_DATA_B: &'static str = include_str!("sample_b.txt");

    type Common = FieldWithLoop;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 10;

    fn common(data: &'static str) -> anyhow::Result<Self::Common> {
        let mut field = data
            .lines()
            .map(|line| field_line.parse(line).map_err(anyhow::Error::msg))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let start_coords = find_and_fix_start(&mut field)?;
        let loop_coords = find_loop(&field, start_coords)?;
        Ok(FieldWithLoop { field, loop_coords })
    }

    fn part_a(FieldWithLoop { loop_coords, .. }: Self::Common) -> anyhow::Result<Self::Answer> {
        Ok(loop_coords.len() / 2)
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
                    (count + matches!(state, InsideLoop) as usize, state)
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
            (OutsideLoop, NorthSouth) => InsideLoop,
            (OutsideLoop, NorthEast) => InsideBottomPipe,
            (OutsideLoop, SouthEast) => InsideTopPipe,
            (InsideLoop, NorthSouth) => OutsideLoop,
            (InsideLoop, NorthEast) => InsideTopPipe,
            (InsideLoop, SouthEast) => InsideBottomPipe,
            (InsideTopPipe, EastWest) => InsideTopPipe,
            (InsideTopPipe, NorthWest) => InsideLoop,
            (InsideTopPipe, SouthWest) => OutsideLoop,
            (InsideBottomPipe, EastWest) => InsideBottomPipe,
            (InsideBottomPipe, NorthWest) => OutsideLoop,
            (InsideBottomPipe, SouthWest) => InsideLoop,
            (_, Ground) => bail!("Ground should not be passed to ExplorationState::next"),
            (_, Start) => bail!("Start should not be passed to ExplorationState::next"),
            (state, segment) => bail!("Cannot encounter {segment:?} when {state:?}"),
        })
    }
}

fn find_and_fix_start(data: &mut [Vec<PipeSegment>]) -> anyhow::Result<(usize, usize)> {
    let (start_row, start_col) = data
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

    let start_segment = [
        NorthWest, NorthEast, NorthSouth, EastWest, SouthWest, SouthEast,
    ]
    .into_iter()
    .find(|&segment| {
        segment
            .adjacent_diffs()
            .into_iter()
            .flatten()
            .all(|(drow, dcol)| {
                check_add_signed_2d((start_row, start_col), (drow, dcol))
                    .and_then(|coords| get_2d(data, coords))
                    .and_then(|segment| segment.adjacent_diffs())
                    .is_some_and(|diffs| diffs.contains(&(-drow, -dcol)))
            })
    })
    .ok_or_else(|| anyhow!("Can't identify start segment at ({start_row}, {start_col})"))?;

    data[start_row][start_col] = start_segment;
    Ok((start_row, start_col))
}

fn find_loop(
    data: &[Vec<PipeSegment>],
    start_coords: (usize, usize),
) -> anyhow::Result<HashSet<(usize, usize)>> {
    let err = |coords| anyhow!("No connected segments found at {coords:?}");
    let mut loop_coords = HashSet::from([start_coords]);
    let mut prev = start_coords;
    let mut coords = connected_segments(data, start_coords)
        .next() // select arbitrarily
        .ok_or_else(|| err(start_coords))?;
    while coords != start_coords {
        loop_coords.insert(coords);
        (prev, coords) = (
            coords,
            connected_segments(data, coords)
                .find(|&next| next != prev)
                .ok_or_else(|| err(coords))?,
        );
    }
    Ok(loop_coords)
}

fn connected_segments(
    data: &[Vec<PipeSegment>],
    from: (usize, usize),
) -> impl Iterator<Item = (usize, usize)> {
    get_2d(data, from)
        .and_then(|segment| segment.adjacent_diffs())
        .into_iter()
        .flatten()
        .filter_map(move |diff| check_add_signed_2d(from, diff))
}

impl PipeSegment {
    fn adjacent_diffs(self) -> Option<[(isize, isize); 2]> {
        Some(match self {
            NorthWest => [(-1, 0), (0, -1)],
            NorthEast => [(-1, 0), (0, 1)],
            NorthSouth => [(-1, 0), (1, 0)],
            EastWest => [(0, -1), (0, 1)],
            SouthWest => [(0, -1), (1, 0)],
            SouthEast => [(0, 1), (1, 0)],
            Ground => return None,
            Start => return None,
        })
    }
}

fn check_add_signed_2d(
    (row, col): (usize, usize),
    (drow, dcol): (isize, isize),
) -> Option<(usize, usize)> {
    row.checked_add_signed(drow)
        .zip(col.checked_add_signed(dcol))
}

fn get_2d(data: &[Vec<PipeSegment>], (row, col): (usize, usize)) -> Option<PipeSegment> {
    data.get(row)?.get(col).copied()
}

fn field_line(input: &mut &'static str) -> winnow::PResult<Vec<PipeSegment>> {
    repeat(1.., pipe_segment).parse_next(input)
}

fn pipe_segment(input: &mut &'static str) -> winnow::PResult<PipeSegment> {
    any.verify_map(|c| match c {
        '|' => Some(NorthSouth),
        '-' => Some(EastWest),
        'L' => Some(NorthEast),
        'J' => Some(NorthWest),
        '7' => Some(SouthWest),
        'F' => Some(SouthEast),
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
