use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use anyhow::bail;
use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, hex_uint},
    combinator::alt,
    seq, Parser,
};

struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Edge {
    direction: Direction,
    length: u64,
    color: u64,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<Edge>;
    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 62;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 952408144115;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        input
            .lines()
            .map(|line| edge.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(edges: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        calc_lagoon_area(&edges)
    }

    fn part_b(edges: Self::Shared) -> anyhow::Result<Self::Answer> {
        let edges = edges
            .into_iter()
            .map(Edge::fix)
            .collect::<anyhow::Result<Vec<_>>>()?;
        calc_lagoon_area(&edges)
    }
}

fn calc_lagoon_area(edges: &[Edge]) -> Result<u64, anyhow::Error> {
    let mut trench = vec![(0, 0)];
    let mut current = (0, 0);
    for edge in edges.iter() {
        let length = edge.length as i64;
        let (x, y) = current;
        let (dx, dy) = match edge.direction {
            Direction::Up => (length, 0),
            Direction::Down => (-length, 0),
            Direction::Left => (0, -length),
            Direction::Right => (0, length),
        };
        current = (x + dx, y + dy);
        trench.push(current);
    }
    Ok(1 + shoelace_formula(&trench) + edges.iter().map(|e| e.length).sum::<u64>() / 2)
}

impl Edge {
    fn fix(self) -> anyhow::Result<Self> {
        let direction = match self.color & 0b1111 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            last_digit => bail!("Invalid direction: {last_digit}"),
        };
        Ok(Edge {
            direction,
            length: self.color >> 4,
            color: self.color,
        })
    }
}

fn shoelace_formula(coords: &[(i64, i64)]) -> u64 {
    coords
        .iter()
        .copied()
        .chain([coords[0]])
        .tuple_windows()
        .map(|((y1, x1), (y2, x2))| x1 * y2 - x2 * y1)
        .sum::<i64>()
        .unsigned_abs()
        / 2
}

fn edge(input: &mut &'static str) -> winnow::Result<Edge> {
    seq! {Edge {
        direction: alt(('R'.value(Direction::Right), 'L'.value(Direction::Left), 'U'.value(Direction::Up), 'D'.value(Direction::Down))),
        _: ' ',
        length: dec_uint,
        _: " (#",
        color: hex_uint,
        _: ')',
    }}.parse_next(input)
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
