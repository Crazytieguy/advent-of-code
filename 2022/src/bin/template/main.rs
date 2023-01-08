use advent_2022::*;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = &'static str;
    type Answer = u32;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 0;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        Ok(("", data))
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        todo!("{data}")
    }

    fn b(_: Self::Parsed) -> Self::Answer {
        0
    }
}
