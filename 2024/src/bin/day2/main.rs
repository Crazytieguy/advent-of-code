#![feature(iter_map_windows)]
use std::borrow::Cow;

use advent_2024::{BasicSolution, Solution};
use winnow::{Parser, ascii::dec_int, combinator::separated};

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<Vec<i8>>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 2;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 4;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        input
            .lines()
            .map(|line| line_parser.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(shared.iter().filter(|&line| is_safe_report(line)).count())
    }

    fn part_b(shared: Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(shared
            .iter()
            .filter(|line| is_safe_report_dampened(line))
            .count())
    }
}

fn is_safe_report_dampened(levels: &[i8]) -> bool {
    (0..levels.len()).any(|i| {
        let with_i_skipped = levels.iter().take(i).chain(levels.iter().skip(i + 1));
        is_safe_report(with_i_skipped)
    })
}

fn is_safe_report<'a>(levels: impl IntoIterator<Item = &'a i8>) -> bool {
    let mut diff_sign = None;
    let is_safe_diff = |diff: i8| {
        1 <= diff.abs()
            && diff.abs() <= 3
            && diff.signum() == *diff_sign.get_or_insert(diff.signum())
    };
    levels
        .into_iter()
        .map_windows(|&[a, b]| b - a)
        .all(is_safe_diff)
}

fn line_parser(input: &mut &'static str) -> winnow::Result<Vec<i8>> {
    separated(1.., dec_int::<_, i8, _>, ' ').parse_next(input)
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
