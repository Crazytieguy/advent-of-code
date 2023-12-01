use advent_2023::*;
use nom::{
    character::complete::{line_ending, not_line_ending},
    multi::separated_list1,
};

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<&'static str>;
    type Answer = u32;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 0;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, not_line_ending)(data)
    }

    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        todo!("{data:?}")
    }

    fn b(_: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(0)
    }
}
