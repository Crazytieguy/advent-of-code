use std::iter;

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

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<Operation>;
    type Answer = String;
    type TestAnswer = &'static str;
    const SAMPLE_ANSWER_A: Self::TestAnswer = "13140";
    const SAMPLE_ANSWER_B: Self::TestAnswer = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    fn parse(data: &'static str) -> IResult<'static, Self::Parsed> {
        separated_list1(line_ending, parse_operation)(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        iter_register(&data)
            .zip(1..)
            .filter(|(_, cycle)| [20, 60, 100, 140, 180, 220].contains(cycle))
            .map(|(reg_x, cycle)| reg_x * cycle)
            .sum::<i32>()
            .to_string()
    }

    fn b(data: Self::Parsed) -> Self::Answer {
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
