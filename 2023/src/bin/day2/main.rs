#![warn(clippy::pedantic)]
use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use winnow::{
    ascii::dec_uint,
    combinator::{alt, fold_repeat, opt, preceded, separated_pair},
    Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct Game {
    id: u8,
    revealed: [u8; 3],
}

const RED: usize = 0;
const GREEN: usize = 1;
const BLUE: usize = 2;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<Game>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 2286;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        input
            .lines()
            .map(|line| game.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(games: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(games
            .iter()
            .filter(|game| {
                let [red, green, blue] = game.revealed;
                red <= 12 && green <= 13 && blue <= 14
            })
            .map(|game| u32::from(game.id))
            .sum())
    }

    fn part_b(games: Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(games
            .into_iter()
            .map(|game| {
                let [red, green, blue] = game.revealed;
                u32::from(red) * u32::from(green) * u32::from(blue)
            })
            .sum())
    }
}

fn game(input: &mut &str) -> winnow::PResult<Game> {
    ("Game ", dec_uint, ": ", revealed)
        .map(|(_, id, _, revealed)| Game { id, revealed })
        .parse_next(input)
}

fn revealed(input: &mut &str) -> winnow::PResult<[u8; 3]> {
    fold_repeat(
        1..,
        preceded(opt(alt((", ", "; "))), color_count),
        || [0; 3],
        |mut acc, (n, color)| {
            acc[color] = acc[color].max(n);
            acc
        },
    )
    .parse_next(input)
}

fn color_count(input: &mut &str) -> winnow::PResult<(u8, usize)> {
    separated_pair(
        dec_uint,
        ' ',
        alt(("red".value(RED), "green".value(GREEN), "blue".value(BLUE))),
    )
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
