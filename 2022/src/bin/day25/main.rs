use advent_2022::*;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    multi::fold_many1,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = i64;
    type Answer = String;
    type TestAnswer = &'static str;
    const SAMPLE_ANSWER_A: Self::TestAnswer = "2=-1=0";
    const SAMPLE_ANSWER_B: Self::TestAnswer = "";

    fn parse(data: &str) -> IResult<Self::Parsed> {
        fold_many1(snafu.terminated(line_ending), || 0, |acc, cur| acc + cur)(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        to_snafu(data)
    }

    fn b(_data: Self::Parsed) -> Self::Answer {
        "".into()
    }
}

fn snafu_digit(input: &str) -> IResult<i64> {
    alt((
        char('2').value(2),
        char('1').value(1),
        char('0').value(0),
        char('-').value(-1),
        char('=').value(-2),
    ))(input)
}

fn snafu(input: &str) -> IResult<i64> {
    fold_many1(snafu_digit, || 0, |acc, cur| acc * 5 + cur)(input)
}

fn to_snafu(number: i64) -> String {
    itertools::unfold(number, |number| {
        if *number == 0 {
            return None;
        }
        let digit_value = (((*number % 5) + 2) % 5) - 2;
        *number -= digit_value;
        *number /= 5;
        Some(match digit_value {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
    })
    .collect::<String>()
    .chars()
    .rev()
    .collect()
}
