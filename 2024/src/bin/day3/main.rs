use std::borrow::Cow;

use advent_2024::{BasicSolution, Solution};
use winnow::{
    Parser,
    ascii::dec_uint,
    combinator::{alt, iterator, seq},
    token::any,
};

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const SAMPLE_INPUT_B: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    type Shared = &'static str;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 161;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 48;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        Ok(input)
    }

    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        let mul_or_skip = alt((mul.map(Some), any.value(None)));
        Ok(iterator(*shared, mul_or_skip)
            .flatten()
            .map(|(a, b)| a * b)
            .sum())
    }

    fn part_b(shared: Self::Shared) -> anyhow::Result<Self::Answer> {
        use Statement::*;
        let statement = alt((mul.map(Mul), "do()".value(Do), "don't()".value(Dont)));
        let statement_or_skip = alt((statement.map(Some), any.value(None)));
        let mut enabled = true;
        Ok(iterator(shared, statement_or_skip)
            .flatten()
            .filter_map(|s| {
                match s {
                    Do => enabled = true,
                    Dont => enabled = false,
                    Mul((a, b)) if enabled => return Some(a * b),
                    _ => {}
                }
                None
            })
            .sum())
    }
}

#[derive(Clone, Copy)]
enum Statement {
    Mul((u32, u32)),
    Do,
    Dont,
}

fn mul(input: &mut &'static str) -> winnow::Result<(u32, u32)> {
    seq! {(_: "mul(", dec_uint, _: ',', dec_uint, _: ')')}.parse_next(input)
}

fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_part_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_part_b()
    }
}
