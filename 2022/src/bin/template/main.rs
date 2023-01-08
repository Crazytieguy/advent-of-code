use advent_2022::*;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = &'static str;
    type A = u32;
    type B = u32;
    const SAMPLE_ANSWER_A: Self::TestA = 0;
    const SAMPLE_ANSWER_B: Self::TestB = 0;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        Ok(("", data))
    }

    fn a(data: Self::Parsed) -> Self::A {
        todo!("{data}")
    }

    fn b(_: Self::Parsed) -> Self::B {
        0
    }
}
