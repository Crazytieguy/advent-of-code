#![warn(clippy::pedantic)]
use std::{borrow::Cow, collections::HashMap};

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;
use itertools::process_results;
use num::Integer;
use winnow::{
    ascii::alphanumeric1,
    combinator::{alt, opt, repeat, separated},
    seq, Parser,
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
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample_a.txt");
    const SAMPLE_INPUT_B: &'static str = include_str!("sample_b.txt");

    type Shared = Maps<'static>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 2;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 6;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        maps.parse(input).map_err(anyhow::Error::msg)
    }

    fn part_a(maps: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        maps.count_steps("AAA", |node| node == "ZZZ")
    }

    fn part_b(maps: Self::Shared) -> anyhow::Result<Self::Answer> {
        process_results(
            maps.network
                .keys()
                .filter(|k| k.ends_with('A'))
                .map(|start| maps.count_steps(start, |node| node.ends_with('Z'))),
            |it| {
                it.reduce(|a, b| a.lcm(&b))
                    .ok_or_else(|| anyhow!("Less than two starting nodes"))
            },
        )?
    }
}

impl Maps<'_> {
    fn count_steps(&self, start: &str, target: impl Fn(&str) -> bool) -> anyhow::Result<usize> {
        let mut current = start;
        self.instructions
            .iter()
            .cycle()
            .take(1_000_000)
            .position(|direction| {
                let (left, right) = &self.network[current];
                current = match direction {
                    Direction::Left => left,
                    Direction::Right => right,
                };
                target(current)
            })
            .map(|c| c + 1)
            .ok_or_else(|| anyhow!("Cycle starting at {start} not found after 1_000_000 steps"))
    }
}

fn maps<'a>(input: &mut &'a str) -> winnow::PResult<Maps<'a>> {
    seq! {Maps {
        instructions: instructions,
        _: "\n\n",
        network: network,
        _: opt('\n'),
    }}
    .parse_next(input)
}

fn network<'a>(input: &mut &'a str) -> winnow::PResult<HashMap<&'a str, (&'a str, &'a str)>> {
    separated(1.., node_targets, '\n').parse_next(input)
}

fn node_targets<'a>(input: &mut &'a str) -> winnow::PResult<(&'a str, (&'a str, &'a str))> {
    seq! {(
        alphanumeric1,
        _: " = ",
        seq!{(_: '(', alphanumeric1, _: ", ", alphanumeric1, _: ')')},
    )}
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
