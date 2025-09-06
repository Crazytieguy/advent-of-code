use advent_2022::*;
use itertools::process_results;
use nom::{
    character::complete::{char, u16, u8},
    sequence::separated_pair,
    Parser,
};

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = (BitGrid, usize);
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 24;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 93;

    fn parse(data: &str) -> IResult<'_, Self::Parsed> {
        let mut rocks = BitGrid::new();
        let mut max_y = 0;
        for line in data.lines() {
            process_results(line.split(" -> ").map(parse_coords), |it| {
                it.map(|(_, xy)| xy).reduce(|(x1, y1), (x2, y2)| {
                    max_y = max_y.max(y1).max(y2);
                    for x in x1.min(x2)..=x1.max(x2) {
                        for y in y1.min(y2)..=y1.max(y2) {
                            rocks.insert(x, y);
                        }
                    }
                    (x2, y2)
                })
            })?;
        }
        Ok(("", (rocks, max_y)))
    }

    fn a((rocks, max_y): Self::Parsed) -> Self::Answer {
        solve::<false>(rocks, max_y)
    }

    fn b((rocks, max_y): Self::Parsed) -> Self::Answer {
        solve::<true>(rocks, max_y)
    }
}

#[derive(Debug, Clone)]
struct BitGrid([[u32; 5]; 320]);

fn solve<const SOLID_FLOOR: bool>(mut taken_coords: BitGrid, max_y: usize) -> usize {
    let num_rocks = taken_coords.len();
    drop_sand::<SOLID_FLOOR>(&mut taken_coords, max_y + 2, 500, 0);
    taken_coords.len() - num_rocks
}

fn parse_coords(data: &str) -> IResult<'_, (usize, usize)> {
    separated_pair(u16.map(usize::from), char(','), u8.map(usize::from))(data)
}

fn drop_sand<const SOLID_FLOOR: bool>(
    taken_coords: &mut BitGrid,
    floor: usize,
    x: usize,
    y: usize,
) -> bool {
    if y == floor {
        return true;
    }
    if taken_coords.contains(x, y) {
        return false;
    }
    for x in [x, x - 1, x + 1] {
        if drop_sand::<SOLID_FLOOR>(taken_coords, floor, x, y + 1) && !SOLID_FLOOR {
            return true;
        }
    }
    taken_coords.insert(x, y);
    false
}

impl BitGrid {
    fn new() -> Self {
        Self([[0; 5]; 320])
    }

    fn len(&self) -> usize {
        self.0
            .into_iter()
            .flatten()
            .map(|x| x.count_ones() as usize)
            .sum()
    }

    fn insert(&mut self, x: usize, y: usize) {
        self.0[x + 160 - 500][y / 32] |= 1 << (y % 32);
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        self.0[x + 160 - 500][y / 32] & (1 << (y % 32)) != 0
    }
}
