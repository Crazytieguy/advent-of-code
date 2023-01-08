use advent_2022::*;
use itertools::Itertools;
use nom::character::complete::{line_ending, u32};
use nom::multi::{fold_many1, separated_list1};
use nom_supreme::ParserExt;
use std::cmp::Reverse;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<u32>;
    type A = u32;
    type B = u32;
    const SAMPLE_ANSWER_A: Self::TestA = 24000;
    const SAMPLE_ANSWER_B: Self::TestB = 45000;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        separated_list1(
            line_ending,
            fold_many1(
                u32.terminated(line_ending),
                || 0,
                |total, item| total + item,
            ),
        )(data)
    }

    fn a(data: Self::Parsed) -> Self::A {
        data.into_iter().max().expect("At least one elf")
    }

    fn b(data: Self::Parsed) -> Self::B {
        data.into_iter()
            .sorted_unstable_by_key(|&cals| Reverse(cals))
            .take(3)
            .sum()
    }
}
