use std::collections::HashMap;

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;
use num::Integer;
use winnow::{
    ascii::alphanumeric1,
    combinator::{alt, delimited, opt, repeat, separated, separated_pair},
    Parser,
};

struct Day;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Maps<'a> {
    network: HashMap<&'a str, (&'a str, &'a str)>,
    instructions: Vec<Direction>,
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample_a.txt");
    const SAMPLE_DATA_B: &'static str = include_str!("sample_b.txt");

    type Common = Maps<'static>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 2;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 6;

    fn common(data: &'static str) -> anyhow::Result<Self::Common> {
        maps.parse(data).map_err(anyhow::Error::msg)
    }

    fn part_a(data: Self::Common) -> anyhow::Result<Self::Answer> {
        Ok(data.count_steps("AAA", |node| node == "ZZZ"))
    }

    fn part_b(data: Self::Common) -> anyhow::Result<Self::Answer> {
        data.network
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|start| data.count_steps(start, |node| node.ends_with('Z')))
            .reduce(|a, b| a.lcm(&b))
            .ok_or_else(|| anyhow!("Less than two starting nodes"))
    }
}

impl<'a> Maps<'a> {
    fn count_steps(&self, start: &str, target: impl Fn(&str) -> bool) -> usize {
        let mut current = start;
        self.instructions
            .iter()
            .cycle()
            .position(|direction| {
                let (left, right) = &self.network[current];
                current = match direction {
                    Direction::Left => left,
                    Direction::Right => right,
                };
                target(current)
            })
            .expect("cycle should never end")
            + 1
    }
}

fn maps<'a>(input: &mut &'a str) -> winnow::PResult<Maps<'a>> {
    (instructions, "\n\n", network, opt('\n'))
        .map(|(instructions, _, network, _)| Maps {
            network,
            instructions,
        })
        .parse_next(input)
}

fn network<'a>(input: &mut &'a str) -> winnow::PResult<HashMap<&'a str, (&'a str, &'a str)>> {
    separated(1.., node_targets, '\n').parse_next(input)
}

fn node_targets<'a>(input: &mut &'a str) -> winnow::PResult<(&'a str, (&'a str, &'a str))> {
    separated_pair(
        alphanumeric1,
        " = ",
        delimited('(', separated_pair(alphanumeric1, ", ", alphanumeric1), ')'),
    )
    .parse_next(input)
}

fn instructions(input: &mut &str) -> winnow::PResult<Vec<Direction>> {
    repeat(1.., direction).parse_next(input)
}

fn direction(input: &mut &str) -> winnow::PResult<Direction> {
    alt(('L'.value(Direction::Left), 'R'.value(Direction::Right))).parse_next(input)
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
