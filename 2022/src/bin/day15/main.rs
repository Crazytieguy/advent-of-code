use advent_2022::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    sequence::tuple,
};
use nom_supreme::ParserExt;
use std::ops::Range;

boilerplate!(Day);

impl Solution for Day {
    type Parsed = Vec<Pair>;
    type Answer = i64;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 26;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 56_000_011;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, pair)(data)
    }

    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest> {
        Self::parse(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        solve_a::<2_000_000>(data) as i64
    }

    fn a_test(data: Self::Parsed) -> Self::Answer {
        solve_a::<10>(data) as i64
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        solve_b::<4_000_000>(data)
    }

    fn b_test(data: Self::Parsed) -> Self::Answer {
        solve_b::<20>(data)
    }
}

type Pair = (i32, i32, i32, i32);

fn solve_a<const EXAMPLE_ROW: i32>(pairs: Vec<Pair>) -> usize {
    let count_covered_xs: usize = pairs
        .iter()
        .flat_map(|&pair| covered_xs(pair, EXAMPLE_ROW))
        .sorted_unstable_by_key(|range| range.start)
        .coalesce(|a, b| {
            if a.end >= b.start {
                Ok(a.start..b.end.max(a.end))
            } else {
                Err((a, b))
            }
        })
        .map(|xs| xs.len())
        .sum();
    let blocked_xs = pairs
        .into_iter()
        .flat_map(|(sx, sy, bx, by)| [(sx, sy), (bx, by)])
        .filter(|&(_, y)| y == EXAMPLE_ROW)
        .unique()
        .count();
    count_covered_xs - blocked_xs
}

fn solve_b<const MAX_COORD: i32>(mut pairs: Vec<Pair>) -> i64 {
    pairs.sort_unstable();
    for y in 0..=MAX_COORD {
        let first_available_x = pairs
            .iter()
            .flat_map(|&pair| covered_xs(pair, y))
            .fold(0, |x, range| if range.contains(&x) { range.end } else { x });
        if first_available_x <= MAX_COORD {
            return first_available_x as i64 * 4_000_000 + y as i64;
        }
    }
    unreachable!("no solution found")
}

fn pair(input: &str) -> IResult<Pair> {
    tuple((
        tag("Sensor at x=").precedes(i32),
        tag(", y=").precedes(i32),
        tag(": closest beacon is at x=").precedes(i32),
        tag(", y=").precedes(i32),
    ))(input)
}

fn covered_xs((sensor_x, sensor_y, beacon_x, beacon_y): Pair, row: i32) -> Option<Range<i32>> {
    let manhattan_distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
    let x_offset = manhattan_distance - (sensor_y - row).abs();
    Some(sensor_x - x_offset..sensor_x + x_offset + 1).filter(|r| !r.is_empty())
}
