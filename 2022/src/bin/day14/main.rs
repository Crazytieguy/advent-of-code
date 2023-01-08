use advent_2022::*;
use itertools::process_results;
use nom::{
    character::complete::{char, u16, u8},
    sequence::separated_pair,
};
use std::collections::HashSet;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = HashSet<(u16, u8)>;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 24;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 93;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        let mut rocks = HashSet::new();
        for line in data.lines() {
            process_results(line.split(" -> ").map(parse_coords), |it| {
                it.map(|(_, xy)| xy).reduce(|(x1, y1), (x2, y2)| {
                    for x in x1.min(x2)..=x1.max(x2) {
                        for y in y1.min(y2)..=y1.max(y2) {
                            rocks.insert((x, y));
                        }
                    }
                    (x2, y2)
                })
            })?;
        }
        Ok(("", rocks))
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        solve::<false>(data)
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        solve::<true>(data)
    }
}

fn solve<const SOLID_FLOOR: bool>(mut taken_coords: HashSet<(u16, u8)>) -> usize {
    let num_rocks = taken_coords.len();
    let max_y = taken_coords
        .iter()
        .map(|&(_, y)| y)
        .max()
        .expect("At least one rock");
    drop_sand::<SOLID_FLOOR>(&mut taken_coords, max_y + 2, 500, 0);
    taken_coords.len() - num_rocks
}

fn parse_coords(data: &str) -> IResult<(u16, u8)> {
    separated_pair(u16, char(','), u8)(data)
}

fn drop_sand<const SOLID_FLOOR: bool>(
    taken_coords: &mut HashSet<(u16, u8)>,
    floor: u8,
    x: u16,
    y: u8,
) -> bool {
    if y == floor {
        return true;
    }
    if taken_coords.contains(&(x, y)) {
        return false;
    }
    for x in [x, x - 1, x + 1] {
        if drop_sand::<SOLID_FLOOR>(taken_coords, floor, x, y + 1) && !SOLID_FLOOR {
            return true;
        }
    }
    taken_coords.insert((x, y));
    false
}
