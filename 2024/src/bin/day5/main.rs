use std::{borrow::Cow, cmp::Ordering};

use advent_2024::{BasicSolution, Solution};
use fxhash::FxHashSet;
use winnow::{
    Parser,
    ascii::dec_uint,
    combinator::{opt, separated, separated_pair, seq},
};

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = SafetyManual;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 143;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 123;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        safety_manual.parse(input).map_err(anyhow::Error::msg)
    }

    fn part_a(safety_manual: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(safety_manual
            .updates
            .iter()
            .filter(|update| {
                update.is_sorted_by(|&a, &b| safety_manual.ordering_rules.contains(&(a, b)))
            })
            .map(|update| update[update.len() / 2] as u32)
            .sum())
    }

    fn part_b(safety_manual: Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(safety_manual
            .updates
            .into_iter()
            .filter(|update| {
                !update.is_sorted_by(|&a, &b| safety_manual.ordering_rules.contains(&(a, b)))
            })
            .map(|mut update| {
                update.sort_unstable_by(|&a, &b| {
                    if safety_manual.ordering_rules.contains(&(a, b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                update[update.len() / 2] as u32
            })
            .sum())
    }
}

#[derive(Clone, Debug)]
struct SafetyManual {
    ordering_rules: FxHashSet<(u8, u8)>,
    updates: Vec<Vec<u8>>,
}

fn safety_manual(input: &mut &'static str) -> winnow::Result<SafetyManual> {
    seq! {SafetyManual {
        ordering_rules: separated(1.., ordering_rule, '\n'),
        _: "\n\n",
        updates: separated(1.., update, '\n'),
        _: opt('\n')
    }}
    .parse_next(input)
}

fn update(input: &mut &'static str) -> winnow::Result<Vec<u8>> {
    separated(1.., dec_uint::<_, u8, _>, ',').parse_next(input)
}

fn ordering_rule(input: &mut &'static str) -> winnow::Result<(u8, u8)> {
    separated_pair(dec_uint, '|', dec_uint).parse_next(input)
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
