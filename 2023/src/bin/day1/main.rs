#![warn(clippy::pedantic)]
use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample_a.txt");
    const SAMPLE_INPUT_B: &'static str = include_str!("sample_b.txt");

    type Common = &'static str;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 142;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 281;

    fn common(input: &'static str) -> anyhow::Result<Self::Common> {
        Ok(input)
    }

    fn part_a(document: Cow<Self::Common>) -> anyhow::Result<Self::Answer> {
        solve(&document, &[])
    }

    fn part_b(document: Self::Common) -> anyhow::Result<Self::Answer> {
        solve(
            document,
            &[
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ],
        )
    }
}

fn solve(document: &str, spelled_out_vals: &[(&str, u32)]) -> anyhow::Result<u32> {
    let calibration_value = |line: &str| {
        let err = || anyhow!("Couldn't find a digit in line '{line}'");

        let digit_at_i = |i| {
            let literal = line[i..=i].parse().ok();
            let match_spelled = |&(digit, val)| line[i..].starts_with(digit).then_some(val);
            literal.or_else(|| spelled_out_vals.iter().find_map(match_spelled))
        };

        let first = (0..line.len()).find_map(digit_at_i).ok_or_else(err)?;
        let last = (0..line.len()).rev().find_map(digit_at_i).ok_or_else(err)?;

        Ok(first * 10 + last)
    };
    itertools::process_results(document.lines().map(calibration_value), |it| it.sum())
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
