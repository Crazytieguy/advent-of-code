use std::cmp::Ordering;

use advent_2022::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, line_ending, u8},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    Parser,
};
use nom_supreme::ParserExt;
use Value::*;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<(Value, Value)>;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 13;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 140;

    fn parse(data: &'static str) -> IResult<'static, Self::Parsed> {
        separated_list1(
            line_ending,
            separated_pair(value, line_ending, value).terminated(line_ending),
        )(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        data.into_iter()
            .enumerate()
            .filter(|(_, (a, b))| b >= a)
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        let divider_a = List(vec![List(vec![Integer(2)])]);
        let divider_b = List(vec![List(vec![Integer(6)])]);
        let all_packets = data
            .iter()
            .flat_map(|(a, b)| [a, b])
            .chain([&divider_a, &divider_b])
            .sorted_unstable()
            .collect_vec();
        all_packets.partition_point(|&v| v <= &divider_a)
            * all_packets.partition_point(|&v| v <= &divider_b)
    }
}

#[derive(Debug, Clone)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Integer(a), list) => List(vec![Integer(*a)]).cmp(list),
            (list, Integer(b)) => list.cmp(&List(vec![Integer(*b)])),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn value(data: &str) -> IResult<Value> {
    alt((
        u8.map(Integer),
        delimited(char('['), separated_list0(char(','), value), char(']')).map(List),
    ))(data)
}
