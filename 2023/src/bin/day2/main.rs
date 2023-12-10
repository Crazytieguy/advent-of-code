#![warn(clippy::pedantic)]
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
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Common = Vec<Game>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 2286;

    fn common(data: &'static str) -> anyhow::Result<Self::Common> {
        data.lines()
            .map(|line| game.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(data: Self::Common) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .filter(|game| {
                let [red, green, blue] = game.revealed;
                red <= 12 && green <= 13 && blue <= 14
            })
            .map(|game| u32::from(game.id))
            .sum())
    }

    fn part_b(data: Self::Common) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .map(|game| {
                let [red, green, blue] = game.revealed;
                u32::from(red) * u32::from(green) * u32::from(blue)
            })
            .sum())
    }
}

fn game(data: &mut &str) -> winnow::PResult<Game> {
    ("Game ", dec_uint, ": ", revealed)
        .map(|(_, id, _, revealed)| Game { id, revealed })
        .parse_next(data)
}

fn revealed(data: &mut &str) -> winnow::PResult<[u8; 3]> {
    fold_repeat(
        1..,
        preceded(opt(alt((", ", "; "))), color_count),
        || [0; 3],
        |mut acc, (n, color)| {
            acc[color] = acc[color].max(n);
            acc
        },
    )
    .parse_next(data)
}

fn color_count(data: &mut &str) -> winnow::PResult<(u8, usize)> {
    separated_pair(
        dec_uint,
        ' ',
        alt(("red".value(RED), "green".value(GREEN), "blue".value(BLUE))),
    )
    .parse_next(data)
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
