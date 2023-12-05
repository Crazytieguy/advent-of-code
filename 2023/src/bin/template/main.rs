use advent_2023::{BasicSolution, Solution};
use winnow::{
    ascii::{line_ending, not_line_ending},
    combinator::{opt, repeat, terminated},
    Parser,
};

struct Day;

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Vec<&'static str>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 0;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        repeat(1.., terminated(line, opt(line_ending)))
            .parse(data)
            .map_err(anyhow::Error::msg)
    }

    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        todo!("{data:?}")
    }

    fn b(_: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(0)
    }
}

fn line(data: &mut &'static str) -> winnow::PResult<&'static str> {
    not_line_ending.parse_next(data)
}

fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_b()
    }
}
