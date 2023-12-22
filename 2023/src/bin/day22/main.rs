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
            if find_supported::<true>(bricks, grid, ends, brick_id).is_empty() {
                total += 1;
            }
        }
        Ok(total)
    }

    fn part_b((bricks, grid, end): Self::Shared) -> anyhow::Result<Self::Answer> {
        let supported = (1..=bricks.len() as u16)
            .map(|brick_id| find_supported::<false>(&bricks, &grid, &end, brick_id))
            .collect_vec();
        let mut initial_supported_counts = vec![0; bricks.len()];
        for supported_id in supported.iter().flatten() {
            initial_supported_counts[*supported_id as usize - 1] += 1;
        }
        for (x, y) in (0..10).cartesian_product(0..10) {
            if grid[1][x][y] != 0 {
                initial_supported_counts[grid[1][x][y] as usize - 1] += 1;
            }
        }
        Ok((1..=bricks.len() as u16)
            .map(|remove| {
                let mut supported_counts = initial_supported_counts.clone();
                chain_reaction(&supported, &mut supported_counts, remove);
                supported_counts.iter().filter(|&&count| count == 0).count()
            })
            .sum())
    }
}

fn chain_reaction(supported: &[FxHashSet<u16>], supported_counts: &mut [usize], remove: u16) {
    for &supported_id in &supported[remove as usize - 1] {
        supported_counts[supported_id as usize - 1] -= 1;
        if supported_counts[supported_id as usize - 1] == 0 {
            chain_reaction(supported, supported_counts, supported_id);
        }
    }
}

fn find_supported<const EXCLUSIVE: bool>(
    bricks: &[Brick],
    grid: &Grid,
    ends: &Ends,
    brick_id: u16,
) -> FxHashSet<u16> {
    let mut supported_ids = FxHashSet::default();
    let brick = &bricks[brick_id as usize - 1];
    let end = ends[brick_id as usize - 1];
    if let Some(floor_above) = grid.get(end as usize + 1) {
        for (x, y) in brick.xys() {
            let supported_id = floor_above[x][y];
            if supported_id != 0 {
                let should_insert = !EXCLUSIVE || {
                    let supported_brick = &bricks[supported_id as usize - 1];
                    supported_brick.xys().all(|(x, y)| {
                        let supported_by = &grid[end as usize][x][y];
                        [0, brick_id].contains(supported_by)
                    })
                };
                if should_insert {
                    supported_ids.insert(supported_id);
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
                let collision = grid
                    .get(start as usize)
                    .is_some_and(|floor| brick.xys().any(|(x, y)| floor[x][y] != 0));
                !collision
            })
            .last()
            .ok_or_else(|| anyhow!("No start found for brick {brick:?}"))?;
        let end = start + brick.to.2 - brick.from.2;
        ends.push(end);
        while grid.len() <= end as usize {
            grid.push(Default::default());
        }
        for z in start..=end {
            for (x, y) in brick.xys() {
                grid[z as usize][x][y] = brick_id;
            }
        }
    }
    Ok((grid, ends))
}

impl Brick {
    fn xys(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (self.from.0 as usize..=self.to.0 as usize)
            .cartesian_product(self.from.1 as usize..=self.to.1 as usize)
    }
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
