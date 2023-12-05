#![warn(clippy::pedantic)]
use advent_2023::{BasicSolution, Solution};
use itertools::Itertools;
use winnow::{
    ascii::{line_ending, not_line_ending},
    combinator::{opt, preceded, repeat, separated_pair, terminated},
    token::take_until0,
    Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct Card {
    matches: usize,
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Vec<Card>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 13;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 30;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        repeat(1.., terminated(card, opt(line_ending)))
            .parse(data)
            .map_err(anyhow::Error::msg)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn part_a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .map(|card| 2usize.pow(card.matches as u32) / 2)
            .sum())
    }

    fn part_b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut card_copies = vec![1; data.len()];
        data.iter().enumerate().for_each(|(i, card)| {
            let copies_of_cur = card_copies[i];
            card_copies[i + 1..]
                .iter_mut()
                .take(card.matches)
                .for_each(|c| *c += copies_of_cur);
        });
        Ok(card_copies.into_iter().sum())
    }
}

fn card(data: &mut &str) -> winnow::PResult<Card> {
    let (winning_numbers, numbers_i_own) = preceded(
        (take_until0(":"), ':'),
        separated_pair(take_until0("|"), '|', not_line_ending),
    )
    .parse_next(data)?;
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
