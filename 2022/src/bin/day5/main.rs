#![feature(get_many_mut)]
use advent_2022::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{char, line_ending, one_of, u8},
        streaming::not_line_ending,
    },
    multi::{many1_count, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    Parser,
};
use nom_supreme::{multi::parse_separated_terminated, ParserExt};
use std::collections::VecDeque;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = (Stacks, Vec<Instruction>);
    type Answer = String;
    type TestAnswer = &'static str;
    const SAMPLE_ANSWER_A: Self::TestAnswer = "CMZ";
    const SAMPLE_ANSWER_B: Self::TestAnswer = "MCD";

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_pair(stacks, line_ending, instructions)(data)
    }

    fn a((stacks, instructions): Self::Parsed) -> Self::Answer {
        solve::<true>(stacks, &instructions)
    }

    fn b((stacks, instructions): Self::Parsed) -> Self::Answer {
        solve::<false>(stacks, &instructions)
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

type Stacks = Vec<VecDeque<char>>;

fn solve<const REVERSE_ORDER: bool>(mut stacks: Stacks, instructions: &[Instruction]) -> String {
    for &Instruction { amount, from, to } in instructions {
        let (amount, from, to) = (amount as usize, from as usize, to as usize);
        let [from, to] = stacks
            .get_many_mut([from, to])
            .expect("stacks should exist");
        let at = from.len() - amount;
        if REVERSE_ORDER {
            to.extend(from.drain(at..).rev());
        } else {
            to.extend(from.drain(at..));
        }
    }
    get_top_crates(stacks)
}

fn crate_(input: &str) -> IResult<Option<char>> {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let crate_ = delimited(char('['), one_of(UPPERCASE), char(']')).map(Some);
    let not_crate = tag("   ").value(None);
    alt((crate_, not_crate))(input)
}

fn stacks(input: &str) -> IResult<Stacks> {
    let mut stacks = vec![];
    let crate_row = parse_separated_terminated(
        crate_,
        char(' '),
        line_ending,
        || 0,
        |i, opt_crate| {
            if i == stacks.len() {
                stacks.push(VecDeque::new());
            }
            if let Some(c) = opt_crate {
                stacks[i].push_front(c);
            }
            i + 1
        },
    );
    let (input, _) = many1_count(crate_row)(input)?;
    let (input, _) = not_line_ending.terminated(line_ending).parse(input)?;
    Ok((input, stacks))
}

fn instruction(input: &str) -> IResult<Instruction> {
    let (input, (_, amount, _, from, _, to)) =
        tuple((tag("move "), u8, tag(" from "), u8, tag(" to "), u8))(input)?;
    let (from, to) = (from - 1, to - 1);
    Ok((input, Instruction { amount, from, to }))
}

fn instructions(input: &str) -> IResult<Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

fn get_top_crates(stacks: Stacks) -> String {
    stacks
        .into_iter()
        .map(|stack| {
            *stack
                .back()
                .expect("each stack should have at least one crate")
        })
        .collect()
}
