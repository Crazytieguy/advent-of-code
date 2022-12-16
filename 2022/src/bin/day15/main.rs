use std::{error::Error, ops::RangeInclusive};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};
use nom_supreme::ParserExt;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = Vec<Pair>;

#[derive(Debug, Clone, Copy)]
struct Pair {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

fn parse_pair(input: &str) -> IResult<Pair> {
    tuple((
        tag("Sensor at x=").precedes(i32),
        tag(", y=").precedes(i32),
        tag(": closest beacon is at x=").precedes(i32),
        tag(", y=").precedes(i32),
    ))
    .map(|(sensor_x, sensor_y, beacon_x, beacon_y)| Pair {
        sensor: (sensor_x, sensor_y),
        beacon: (beacon_x, beacon_y),
    })
    .parse(input)
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, parse_pair)(data)
}

impl Pair {
    fn covered_xs(&self, row: i32) -> Option<RangeInclusive<i32>> {
        let manhattan_distance =
            (self.sensor.0 - self.beacon.0).abs() + (self.sensor.1 - self.beacon.1).abs();
        let x_offset = manhattan_distance - (self.sensor.1 - row).abs();
        Some(self.sensor.0 - x_offset..=self.sensor.0 + x_offset).filter(|r| !r.is_empty())
    }
}

fn part_a(data: &Parsed, interesting_row: i32) -> i32 {
    let count_covered_xs = data
        .iter()
        .flat_map(|pair| pair.covered_xs(interesting_row))
        .sorted_by_key(|covered_xs| *covered_xs.start())
        .fold((0, i32::MIN), |(count, end), covered_xs| {
            let count_from = (*covered_xs.start()).max(end + 1);
            let added_count = 0.max(*covered_xs.end() + 1 - count_from);
            (count + added_count, end.max(*covered_xs.end()))
        })
        .0;
    let blocked_xs = data
        .iter()
        .flat_map(|&Pair { sensor, beacon }| [sensor, beacon])
        .filter(|&(_, y)| y == interesting_row)
        .unique()
        .count();
    count_covered_xs - blocked_xs as i32
}

fn part_b(data: &Parsed, max_coord: i32) -> i64 {
    let sorted_pairs = {
        let mut pairs = data.clone();
        pairs.sort_by_key(|pair| pair.sensor.0);
        pairs
    };

    for y in 0..=max_coord {
        let mut x = 0;
        for covered_xs in sorted_pairs.iter().flat_map(|pair| pair.covered_xs(y)) {
            if covered_xs.contains(&x) {
                x = covered_xs.end() + 1;
            }
        }
        if x <= max_coord {
            return x as i64 * 4000000 + y as i64;
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
        println!("part a: {}", part_a(&parse(DATA)?.1, 2000000));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1, 20), 56000011);
        println!("part b: {}", part_b(&parse(DATA)?.1, 4000000));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed, 2000000));
    println!("part b: {}", part_b(&parsed, 4000000));
    Ok(())
}
