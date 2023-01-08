use advent_2022::*;
use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;
use std::iter;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<Operation>;
    type A = i32;
    type B = String;
    type TestB = &'static str;
    const SAMPLE_ANSWER_A: Self::TestA = 13140;
    const SAMPLE_ANSWER_B: Self::TestB = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, parse_operation)
            .terminated(line_ending)
            .parse(data)
    }

    fn a(data: Self::Parsed) -> Self::A {
        iter_register(&data)
            .zip(1..)
            .filter(|(_, cycle)| [20, 60, 100, 140, 180, 220].contains(cycle))
            .map(|(reg_x, cycle)| reg_x * cycle)
            .sum()
    }

    fn b(data: Self::Parsed) -> Self::B {
        iter_register(&data)
            .chunks(40)
            .into_iter()
            .flat_map(|row| {
                iter::once('\n').chain(row.zip(0..).map(|(x, pos)| {
                    if x.abs_diff(pos) <= 1 {
                        '#'
                    } else {
                        '.'
                    }
                }))
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Noop,
    Add(i32),
}

use Operation::*;

fn parse_operation(input: &str) -> IResult<Operation> {
    alt((tag("noop").value(Noop), tag("addx ").precedes(i32).map(Add)))(input)
}

fn iter_register(data: &[Operation]) -> impl Iterator<Item = i32> + '_ {
    data.iter()
        .scan(1, |register, op| {
            Some(repeat_n(
                *register, // dereference before mutating
                match op {
                    Noop => 1,
                    Add(x) => {
                        *register += x;
                        2
                    }
                },
            ))
        })
        .flatten()
}
