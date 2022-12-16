use std::error::Error;

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
    let (input, mut pairs) = separated_list1(line_ending, parse_pair)(data)?;
    pairs.sort_by_key(|&Pair { sensor, .. }| sensor.0);
    Ok((input, pairs))
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part_a(data: &Parsed, interesting_row: i32) -> usize {
    data.iter()
        .flat_map(|&Pair { sensor, beacon }| {
            let covered_distance = manhattan_distance(sensor, beacon);
            let distance_from_interesting_row = (sensor.1 - interesting_row).abs();
            let allowed_x_offset = covered_distance - distance_from_interesting_row;
            (sensor.0 - allowed_x_offset..=sensor.0 + allowed_x_offset).filter(move |&x| {
                (sensor.1 != interesting_row || x != sensor.0)
                    && (beacon.1 != interesting_row || x != beacon.0)
            })
        })
        .unique()
        .count()
}

fn part_b(data: &Parsed, max_coord: i32) -> i64 {
    for y in 0..=max_coord {
        let mut x = 0;
        for &Pair { sensor, beacon } in data {
            let covered_distance = manhattan_distance(sensor, beacon);
            if manhattan_distance(sensor, (x, y)) <= covered_distance {
                x = 1 + sensor.0 + covered_distance - (y - sensor.1).abs();
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
