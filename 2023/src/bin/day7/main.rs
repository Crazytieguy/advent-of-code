#![warn(clippy::pedantic)]
use std::{borrow::Cow, cmp::Reverse};

use advent_2023::{BasicSolution, Solution};
use itertools::Itertools;
use winnow::{ascii::dec_uint, seq, token::any, Parser};

struct Day;

#[derive(Debug, Clone)]
struct Bid {
    hand: [u8; 5],
    amount: u16,
}

// The two top counts define the hand type
type HandType = [u8; 2];

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<Bid>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 6440;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 5905;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        input
            .lines()
            .map(|line| bid.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(bids: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(sum_sortable_bids(bids.iter().map(|bid| {
            (hand_type_part_a(bid.hand), bid.hand, bid.amount)
        })))
    }

    fn part_b(bids: Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(sum_sortable_bids(bids.into_iter().map(|mut bid| {
            jacks_to_jokers(&mut bid.hand);
            (hand_type_part_b(bid.hand), bid.hand, bid.amount)
        })))
    }
}

fn sum_sortable_bids(bid_amounts: impl Iterator<Item = (HandType, [u8; 5], u16)>) -> u32 {
    bid_amounts
        .sorted_unstable()
        .zip(1..)
        .map(|((_, _, amount), i)| i * u32::from(amount))
        .sum()
}

fn hand_type_part_a(hand: [u8; 5]) -> HandType {
    let mut counts = card_counts(hand);
    counts.sort_unstable_by_key(|&c| Reverse(c));
    let [first, second, ..] = counts;
    [first, second]
}

fn hand_type_part_b(hand: [u8; 5]) -> HandType {
    let mut counts = card_counts(hand);
    counts[1..].sort_unstable_by_key(|&c| Reverse(c));
    let [jokers, first, second, ..] = counts;
    [jokers + first, second]
}

fn card_counts(hand: [u8; 5]) -> [u8; 13] {
    hand.iter().fold([0; 13], |mut counts, &card| {
        counts[card as usize] += 1;
        counts
    })
}

fn jacks_to_jokers(hand: &mut [u8; 5]) {
    for card in hand.iter_mut() {
        match card {
            0..=8 => *card += 1,
            9 => *card = 0,
            _ => {}
        }
    }
}

fn bid(input: &mut &'static str) -> winnow::PResult<Bid> {
    seq! {Bid {
        hand: (card, card, card, card, card).map(From::from),
        _: ' ',
        amount: dec_uint,
    }}
    .parse_next(input)
}

fn card(input: &mut &'static str) -> winnow::PResult<u8> {
    any.verify_map(|c| match c {
        '2'..='9' => Some(c as u8 - b'2'),
        'T' => Some(8),
        'J' => Some(9),
        'Q' => Some(10),
        'K' => Some(11),
        'A' => Some(12),
        _ => None,
    })
    .parse_next(input)
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
