#![feature(anonymous_lifetime_in_impl_trait)]
use std::{collections::HashSet, str::Lines};

use advent_2022::*;
use itertools::Itertools;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Lines<'static>;
    type Answer = u32;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 157;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 70;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        Ok(("", data.lines()))
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        data.map(|line| line.split_at(line.len() / 2))
            .map(|(left, right)| intersecting_item([left, right]))
            .map(priority)
            .sum()
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        data.chunks(3)
            .into_iter()
            .map(intersecting_item)
            .map(priority)
            .sum()
    }
}

fn priority(item: u8) -> u32 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => panic!("illegal character {item}"),
    }
    .into()
}

fn intersecting_item(group: impl IntoIterator<Item = &str>) -> u8 {
    group
        .into_iter()
        .map(|items| items.bytes().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).copied().collect())
        .expect("there should be more than one set")
        .into_iter()
        .exactly_one()
        .expect("there should be exactly one item in the intersection")
}
