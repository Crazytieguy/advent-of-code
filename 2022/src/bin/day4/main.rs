use advent_2022::*;
use nom::{
    character::complete::{char, line_ending, u32},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;
use std::ops::RangeInclusive;

type RangesPair = [RangeInclusive<u32>; 2];

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<RangesPair>;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 2;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 4;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(
            line_ending,
            u32.separated_array(char('-'))
                .map(|[a, b]| a..=b)
                .separated_array(char(',')),
        )(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        data.into_iter()
            .filter(one_range_contains_the_other)
            .count()
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        data.into_iter().filter(ranges_overlap).count()
    }
}

fn one_range_contains_the_other([a, b]: &RangesPair) -> bool {
    a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) && b.contains(a.end())
}

fn ranges_overlap([a, b]: &RangesPair) -> bool {
    a.contains(b.start()) || a.contains(b.end())
}
