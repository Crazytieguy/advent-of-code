use advent_2023::*;
use itertools::Itertools;
use nom::{
    bytes::complete::{take_until, take_while},
    character::complete::{char, line_ending},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;

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

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, card)(data)
    }

    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .map(|card| 2usize.pow(card.matches as u32) / 2)
            .sum())
    }

    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
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

fn card(data: &str) -> IResult<Card> {
    let (data, _) = take_until(":").and(char(':')).parse(data)?;
    let (data, [winning_numbers, numbers_i_own]) = take_while(|c| matches!(c, '0'..='9' | ' '))
        .separated_array(char('|'))
        .parse(data)?;
    let matches = winning_numbers
        .split_ascii_whitespace()
        .filter(|n| numbers_i_own.split_ascii_whitespace().contains(n))
        .count();
    Ok((data, Card { matches }))
}

fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_b()
    }
}
