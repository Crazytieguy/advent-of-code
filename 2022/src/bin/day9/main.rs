use std::error::Error;

use itertools::{repeat_n, Itertools};
use nom::{
    character::complete::{alpha1, char, line_ending, u8},
    multi::separated_list1,
    sequence::separated_pair,
};
use nom_supreme::ParserExt;
use serde::Deserialize;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Clone, Copy, Deserialize)]
enum Direction {
    #[serde(rename = "R")]
    Right,
    #[serde(rename = "U")]
    Up,
    #[serde(rename = "L")]
    Left,
    #[serde(rename = "D")]
    Down,
}

type Parsed = Vec<(Direction, u8)>;

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(
        line_ending,
        separated_pair(alpha1.map_res(serde_plain::from_str), char(' '), u8),
    )(data)
}

type Point = (i16, i16);

fn follow_knot(leader: Point, follower: &mut Point) {
    let diff = (leader.0 - follower.0, leader.1 - follower.1);
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        follower.0 += diff.0.signum();
        follower.1 += diff.1.signum();
    }
}

fn reposition_head(direction: Direction, head: &mut Point) {
    use Direction::*;
    let diff = match direction {
        Right => (1, 0),
        Up => (0, 1),
        Left => (-1, 0),
        Down => (0, -1),
    };
    head.0 += diff.0;
    head.1 += diff.1;
}

fn solve<const N: usize>(data: &Parsed) -> usize {
    let mut points = [(0, 0); N];
    data.iter()
        .flat_map(|&(direction, amount)| repeat_n(direction, amount as usize))
        .map(|direction| {
            reposition_head(direction, &mut points[0]);
            (0..N - 1).for_each(|i| follow_knot(points[i], &mut points[i + 1]));
            points[N - 1]
        })
        .unique()
        .count()
}

fn part_a(data: &Parsed) -> usize {
    solve::<2>(data)
}

fn part_b(data: &Parsed) -> usize {
    solve::<10>(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");
    const SAMPLE_B: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 13);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_B)?.1), 36);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
