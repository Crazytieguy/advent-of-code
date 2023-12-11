#![warn(clippy::pedantic)]
use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use itertools::Itertools;
use winnow::{combinator::rest, token::take_until0, Parser};

struct Day;

#[derive(Debug, Clone)]
struct Card {
    matches: usize,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Common = Vec<Card>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 13;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 30;

    fn common(input: &'static str) -> anyhow::Result<Self::Common> {
        input
            .lines()
            .map(|line| card.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(cards: Cow<Self::Common>) -> anyhow::Result<Self::Answer> {
        cards
            .iter()
            .map(|card| u32::try_from(card.matches).map(|matches| 2usize.pow(matches) / 2))
            .sum::<Result<usize, _>>()
            .map_err(anyhow::Error::from)
    }

    fn part_b(cards: Self::Common) -> anyhow::Result<Self::Answer> {
        let mut card_copies = vec![1; cards.len()];
        cards.iter().enumerate().for_each(|(i, card)| {
            let copies_of_cur = card_copies[i];
            card_copies[i + 1..]
                .iter_mut()
                .take(card.matches)
                .for_each(|c| *c += copies_of_cur);
        });
        Ok(card_copies.into_iter().sum())
    }
}

fn card(input: &mut &str) -> winnow::PResult<Card> {
    let (_, _, winning_numbers, _, numbers_i_own) =
        (take_until0(":"), ':', take_until0("|"), '|', rest).parse_next(input)?;
    let matches = winning_numbers
        .split_ascii_whitespace()
        .filter(|n| numbers_i_own.split_ascii_whitespace().contains(n))
        .count();
    Ok(Card { matches })
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
