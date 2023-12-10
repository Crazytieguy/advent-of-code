use std::collections::HashSet;

use advent_2023::{BasicSolution, Solution};
use anyhow::{anyhow, bail};
use itertools::Itertools;
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

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");
    const SAMPLE_DATA_B: &'static str = include_str!("sample_b.txt");

    type Parsed = Vec<Vec<PipeSegment>>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 10;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        data.lines()
            .map(|line| line_parser.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let loop_coords = find_loop(&data)?;
        Ok(loop_coords.len() / 2)
    }

    fn part_b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let loop_coords = find_loop(&data)?;
        count_inside_loop(&data, &loop_coords)
    }
}

fn count_inside_loop(
    data: &[Vec<PipeSegment>],
    loop_coords: &HashSet<(usize, usize)>,
) -> anyhow::Result<usize> {
    let count_line_inside_loop = |(row, line): (usize, &Vec<PipeSegment>)| {
        line.iter()
            .enumerate()
            .scan(OutsideLoop, |state, (col, &(mut segment))| {
                if matches!(segment, Start) {
                    segment = match start_to_pipe_segment((row, col), data) {
                        Ok(segment) => segment,
                        Err(e) => return Some(Err(e)),
                    };
                }
                if loop_coords.contains(&(row, col)) {
                    *state = match state.next(segment) {
                        Ok(state) => state,
                        Err(e) => {
                            return Some(Err(
                                e.context(format!("Failed state stransition at ({row}, {col})"))
                            ))
                        }
                    };
                    Some(Ok(0))
                } else if matches!(*state, InsideLoop) {
                    Some(Ok(1))
                } else {
                    Some(Ok(0))
                }
            })
            .sum::<anyhow::Result<usize>>()
    };
    data.iter().enumerate().map(count_line_inside_loop).sum()
}

fn start_to_pipe_segment(
    start_coords: (usize, usize),
    data: &[Vec<PipeSegment>],
) -> anyhow::Result<PipeSegment> {
    let (a, b) = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|&(drow, dcol)| {
            let coords = check_add_signed_2d(start_coords, (drow, dcol));
            coords
                .and_then(|coords| get_2d(data, coords))
                .is_some_and(|segment| {
                    segment
                        .adjacent_diffs()
                        .map_or(false, |diffs| diffs.contains(&(-drow, -dcol)))
                })
        })
        .collect_tuple()
        .ok_or_else(|| anyhow!("Not exactly two pipe segments connect to start"))?;
    Ok(match [a, b] {
        [(-1, 0), (0, -1)] => NorthWest,
        [(-1, 0), (0, 1)] => NorthEast,
        [(-1, 0), (1, 0)] => NorthSouth,
        [(0, -1), (0, 1)] => EastWest,
        [(0, -1), (1, 0)] => SouthWest,
        [(0, 1), (1, 0)] => SouthEast,
        _ => bail!("Weird start connections {a:?} {b:?}"),
    })
}

impl TraversalState {
    /// Assumes we are traversing from west to east,
    /// and only pipe segments from the main loop are passed.
    /// Also - start is replaced by whatever pipe segment it represents
    fn next(self, segment: PipeSegment) -> anyhow::Result<Self> {
        Ok(match (self, segment) {
            (_, Ground) => bail!("Ground should not be passed to ExplorationState::next"),
            (_, Start) => bail!("Start should not be passed to ExplorationState::next"),
            (OutsideLoop, NorthSouth) => InsideLoop,
            (OutsideLoop, NorthEast) => InsideBottomPipe,
            (OutsideLoop, SouthEast) => InsideTopPipe,
            (OutsideLoop, EastWest) => bail!("Cannot encounter EastWest outside loop"),
            (OutsideLoop, NorthWest) => bail!("Cannot encounter NorthWest outside loop"),
            (OutsideLoop, SouthWest) => bail!("Cannot encounter SouthWest outside loop"),
            (InsideLoop, NorthSouth) => OutsideLoop,
            (InsideLoop, NorthEast) => InsideTopPipe,
            (InsideLoop, SouthEast) => InsideBottomPipe,
            (InsideLoop, EastWest) => bail!("Cannot encounter EastWest inside loop"),
            (InsideLoop, NorthWest) => bail!("Cannot encounter NorthWest inside loop"),
            (InsideLoop, SouthWest) => bail!("Cannot encounter SouthWest inside loop"),
            (InsideTopPipe, EastWest) => InsideTopPipe,
            (InsideTopPipe, NorthWest) => InsideLoop,
            (InsideTopPipe, SouthWest) => OutsideLoop,
            (InsideTopPipe, NorthSouth) => bail!("Cannot encounter NorthSouth inside top pipe"),
            (InsideTopPipe, NorthEast) => bail!("Cannot encounter NorthEast inside top pipe"),
            (InsideTopPipe, SouthEast) => bail!("Cannot encounter SouthEast inside top pipe"),
            (InsideBottomPipe, EastWest) => InsideBottomPipe,
            (InsideBottomPipe, NorthWest) => OutsideLoop,
            (InsideBottomPipe, SouthWest) => InsideLoop,
            (InsideBottomPipe, NorthSouth) => {
                bail!("Cannot encounter NorthSouth inside bottom pipe")
            }
            (InsideBottomPipe, NorthEast) => bail!("Cannot encounter NorthEast inside bottom pipe"),
            (InsideBottomPipe, SouthEast) => bail!("Cannot encounter SouthEast inside bottom pipe"),
        })
    }
}

fn find_loop(data: &[Vec<PipeSegment>]) -> Result<HashSet<(usize, usize)>, anyhow::Error> {
    let start_coords = data
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find_map(|(col, &segment)| match segment {
                    Start => Some((row, col)),
                    _ => None,
                })
        })
        .ok_or_else(|| anyhow!("No start found"))?;
    let (mut a, mut b) = (-1..=1)
        .cartesian_product(-1..=1)
        .filter_map(|(drow, dcol)| {
            let coords = check_add_signed_2d(start_coords, (drow, dcol))?;
            if get_2d(data, coords).is_some_and(|segment| {
                segment
                    .adjacent_diffs()
                    .map_or(false, |diffs| diffs.contains(&(-drow, -dcol)))
            }) {
                Some(coords)
            } else {
                None
            }
        })
        .collect_tuple()
        .ok_or_else(|| anyhow!("Not exactly two pipe segments connect to start"))?;
    let mut seen = HashSet::from([start_coords, a, b]);
    while a != b {
        a = find_unseen_connected(data, a, &seen)?;
        b = find_unseen_connected(data, b, &seen)?;
        seen.insert(a);
        seen.insert(b);
    }
    Ok(seen)
}

fn find_unseen_connected(
    data: &[Vec<PipeSegment>],
    from: (usize, usize),
    seen: &HashSet<(usize, usize)>,
) -> Result<(usize, usize), anyhow::Error> {
    get_2d(data, from)
        .and_then(|segment| segment.adjacent_diffs())
        .and_then(|diffs| {
            diffs
                .into_iter()
                .filter_map(|diff| {
                    check_add_signed_2d(from, diff).filter(|&coords| !seen.contains(&coords))
                })
                .exactly_one()
                .ok()
        })
        .ok_or_else(|| anyhow!("No next pip segment found for {from:?}"))
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

impl PipeSegment {
    fn adjacent_diffs(self) -> Option<[(isize, isize); 2]> {
        match self {
            NorthSouth => Some([(-1, 0), (1, 0)]),
            EastWest => Some([(0, 1), (0, -1)]),
            NorthEast => Some([(-1, 0), (0, 1)]),
            NorthWest => Some([(-1, 0), (0, -1)]),
            SouthWest => Some([(1, 0), (0, -1)]),
            SouthEast => Some([(1, 0), (0, 1)]),
            Ground => None,
            Start => None,
        }
    }
}

fn line_parser(input: &mut &'static str) -> winnow::PResult<Vec<PipeSegment>> {
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
