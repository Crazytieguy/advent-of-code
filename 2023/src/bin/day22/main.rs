use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;
use fxhash::FxHashSet;
use itertools::Itertools;
use winnow::{ascii::dec_uint, seq, Parser};

struct Day;

#[derive(Debug, Clone)]
struct Brick {
    from: (u8, u8, u16),
    to: (u8, u8, u16),
}

type Grid = Vec<[[u16; 10]; 10]>;
type Ends = Vec<u16>;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = (Vec<Brick>, Grid, Ends);
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 5;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 7;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        let mut bricks = input
            .lines()
            .map(|line| brick.parse(line).map_err(anyhow::Error::msg))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let (grid, ends) = compute_resting_position(&mut bricks)?;
        Ok((bricks, grid, ends))
    }

    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        let (bricks, grid, ends) = shared.as_ref();
        let mut total = 0;
        for brick_id in 1..=bricks.len() as u16 {
            if find_exclusively_supported(bricks, grid, ends, brick_id).is_empty() {
                total += 1;
            }
        }
        Ok(total)
    }

    fn part_b((bricks, grid, end): Self::Shared) -> anyhow::Result<Self::Answer> {
        let supported = (1..=bricks.len() as u16)
            .map(|brick_id| find_supported(&bricks, &grid, &end, brick_id))
            .collect_vec();
        let mut initial_supported_counts = vec![0; supported.len()];
        for supported_id in supported.iter().flatten() {
            initial_supported_counts[*supported_id as usize - 1] += 1;
        }
        for x in 0..10 {
            for y in 0..10 {
                if grid[1][x][y] != 0 {
                    initial_supported_counts[grid[1][x][y] as usize - 1] += 1;
                }
            }
        }
        Ok((1..=bricks.len() as u16)
            .map(|remove| chain_reaction(&supported, initial_supported_counts.clone(), remove))
            .sum())
    }
}

fn chain_reaction(
    supported: &[FxHashSet<u16>],
    mut supported_counts: Vec<usize>,
    remove: u16,
) -> usize {
    let mut queue = vec![remove];
    let mut seen = FxHashSet::default();
    while let Some(brick_id) = queue.pop() {
        if !seen.insert(brick_id) {
            continue;
        }
        for supported_id in &supported[brick_id as usize - 1] {
            supported_counts[*supported_id as usize - 1] -= 1;
            if supported_counts[*supported_id as usize - 1] == 0 {
                queue.push(*supported_id);
            }
        }
    }
    supported_counts.iter().filter(|&&count| count == 0).count()
}

fn find_supported(bricks: &[Brick], grid: &Grid, ends: &Ends, brick_id: u16) -> FxHashSet<u16> {
    let mut supported_ids = FxHashSet::default();
    let brick = &bricks[brick_id as usize - 1];
    let end = ends[brick_id as usize - 1];
    if let Some(floor_above) = grid.get(end as usize + 1) {
        for x in brick.from.0..=brick.to.0 {
            for y in brick.from.1..=brick.to.1 {
                let supported_id = floor_above[x as usize][y as usize];
                if supported_id != 0 {
                    supported_ids.insert(supported_id);
                }
            }
        }
    }
    supported_ids
}

fn find_exclusively_supported(
    bricks: &[Brick],
    grid: &Grid,
    ends: &Ends,
    brick_id: u16,
) -> FxHashSet<u16> {
    let mut supported_ids = FxHashSet::default();
    let brick = &bricks[brick_id as usize - 1];
    let end = ends[brick_id as usize - 1];
    if let Some(floor_above) = grid.get(end as usize + 1) {
        for x in brick.from.0..=brick.to.0 {
            for y in brick.from.1..=brick.to.1 {
                let supported_id = floor_above[x as usize][y as usize];
                if supported_id != 0 {
                    let supported_brick = &bricks[supported_id as usize - 1];
                    if (supported_brick.from.0..=supported_brick.to.0)
                        .cartesian_product(supported_brick.from.1..=supported_brick.to.1)
                        .all(|(x, y)| {
                            let supported_by = &grid[end as usize][x as usize][y as usize];
                            [0, brick_id].contains(supported_by)
                        })
                    {
                        supported_ids.insert(supported_id);
                    }
                }
            }
        }
    }
    supported_ids
}

fn compute_resting_position(bricks: &mut [Brick]) -> anyhow::Result<(Grid, Vec<u16>)> {
    bricks.sort_unstable_by_key(|brick| brick.from.2);
    let mut grid: Vec<[[u16; 10]; 10]> = vec![];
    let mut ends = vec![];
    for (brick, brick_id) in bricks.iter().zip(1..) {
        let start = (1..=brick.from.2)
            .rev()
            .take_while(|&start| {
                let collision = grid.get(start as usize).is_some_and(|floor| {
                    for x in brick.from.0..=brick.to.0 {
                        for y in brick.from.1..=brick.to.1 {
                            if floor[x as usize][y as usize] != 0 {
                                return true;
                            }
                        }
                    }
                    false
                });
                !collision
            })
            .last()
            .ok_or_else(|| anyhow!("No start found for brick {brick:?}"))?;
        let end = start + brick.to.2 - brick.from.2;
        ends.push(end);
        grid.resize_with(grid.len().max(end as usize + 1), Default::default);
        for z in start..=end {
            for x in brick.from.0..=brick.to.0 {
                for y in brick.from.1..=brick.to.1 {
                    grid[z as usize][x as usize][y as usize] = brick_id;
                }
            }
        }
    }
    Ok((grid, ends))
}

fn brick(input: &mut &'static str) -> winnow::PResult<Brick> {
    seq! {Brick {
        from: coords,
        _: '~',
        to: coords
    }}
    .parse_next(input)
}

fn coords(input: &mut &'static str) -> winnow::PResult<(u8, u8, u16)> {
    seq! { (dec_uint, _: ',', dec_uint, _: ',', dec_uint) }.parse_next(input)
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
