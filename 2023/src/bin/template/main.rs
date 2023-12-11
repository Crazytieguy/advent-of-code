use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use winnow::{combinator::rest, Parser};

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<&'static str>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 0;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        input
            .lines()
            .map(|line| line_parser.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        todo!("{shared:?}")
    }

    fn part_b(_: Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(0)
    }
}

fn line_parser(input: &mut &'static str) -> winnow::PResult<&'static str> {
    rest.parse_next(input)
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
