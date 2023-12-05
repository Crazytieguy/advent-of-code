use advent_2023::{BasicSolution, Solution};
use winnow::{combinator::rest, Parser};

struct Day;

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Vec<&'static str>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 0;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        data.lines()
            .map(|line| line_parser.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        todo!("{data:?}")
    }

    fn part_b(_: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(0)
    }
}

fn line_parser(data: &mut &'static str) -> winnow::PResult<&'static str> {
    rest.parse_next(data)
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
