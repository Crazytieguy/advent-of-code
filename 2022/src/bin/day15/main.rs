use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    sequence::tuple,
};
use nom_supreme::ParserExt;
use std::{error::Error, ops::Range};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Pair = (i32, i32, i32, i32);
type Parsed = Vec<Pair>;

fn parse_pair(input: &str) -> IResult<Pair> {
    tuple((
        tag("Sensor at x=").precedes(i32),
        tag(", y=").precedes(i32),
        tag(": closest beacon is at x=").precedes(i32),
        tag(", y=").precedes(i32),
    ))(input)
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, parse_pair)(data)
}

fn covered_xs((sensor_x, sensor_y, beacon_x, beacon_y): Pair, row: i32) -> Option<Range<i32>> {
    let manhattan_distance = (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs();
    let x_offset = manhattan_distance - (sensor_y - row).abs();
    Some(sensor_x - x_offset..sensor_x + x_offset + 1).filter(|r| !r.is_empty())
}

fn part_a(data: &Parsed, example_row: i32) -> usize {
    let count_covered_xs: usize = data
        .iter()
        .flat_map(|&pair| covered_xs(pair, example_row))
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
    let blocked_xs = data
        .iter()
        .flat_map(|&(sx, sy, bx, by)| [(sx, sy), (bx, by)])
        .filter(|&(_, y)| y == example_row)
        .unique()
        .count();
    count_covered_xs - blocked_xs
}

fn part_b(data: &Parsed, max_coord: i32) -> i64 {
    let sorted_pairs = data.iter().copied().sorted_unstable().collect_vec();
    for y in 0..=max_coord {
        let first_available_x = sorted_pairs
            .iter()
            .flat_map(|&pair| covered_xs(pair, y))
            .fold(0, |x, range| if range.contains(&x) { range.end } else { x });
        if first_available_x <= max_coord {
            return first_available_x as i64 * 4_000_000 + y as i64;
        }
    }
    unreachable!("no solution found")
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1, 10), 26);
        println!("part a: {}", part_a(&parse(DATA)?.1, 2_000_000));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1, 20), 56_000_011);
        println!("part b: {}", part_b(&parse(DATA)?.1, 4_000_000));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed, 2_000_000));
    println!("part b: {}", part_b(&parsed, 4_000_000));
    Ok(())
}
