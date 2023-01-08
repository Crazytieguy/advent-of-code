use advent_2022::*;
use itertools::Itertools;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = &'static str;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 7;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 19;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        Ok(("", data))
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        solve::<4>(data)
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        solve::<14>(data)
    }
}

fn solve<const N: usize>(data: &str) -> usize {
    data.as_bytes()
        .windows(N)
        .position(|window| window.iter().all_unique())
        .expect("There should be an all unique sequence")
        + N
}
