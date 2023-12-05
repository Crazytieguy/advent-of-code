#![warn(clippy::pedantic)]
use advent_2023::{BasicSolution, Solution};
use winnow::{
    ascii::{dec_uint, line_ending},
    combinator::{alt, fold_repeat, opt, preceded, repeat, separated_pair, terminated},
    stream::AsChar,
    token::take_till0,
    Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct Game {
    id: u8,
    revealed: Revealed,
}

#[derive(Debug, Clone, Default)]
struct Revealed {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Clone)]
enum Color {
    Blue,
    Red,
    Green,
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Vec<Game>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 8;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 2286;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        repeat(1.., terminated(game, opt(line_ending)))
            .parse(data)
            .map_err(anyhow::Error::msg)
    }

    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .filter(|game| {
                let Revealed { red, green, blue } = game.revealed;
                red <= 12 && green <= 13 && blue <= 14
            })
            .map(|game| u32::from(game.id))
            .sum())
    }

    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .map(|game| {
                let Revealed { red, green, blue } = game.revealed;
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

fn revealed(data: &mut &str) -> winnow::PResult<Revealed> {
    fold_repeat(
        1..,
        preceded(take_till0(AsChar::is_dec_digit), color_count),
        Revealed::default,
        |mut acc, (n, color)| {
            match color {
                Color::Red => acc.red = acc.red.max(n),
                Color::Green => acc.green = acc.green.max(n),
                Color::Blue => acc.blue = acc.blue.max(n),
            }
            acc
        },
    )
    .parse_next(data)
}

fn color_count(data: &mut &str) -> winnow::PResult<(u8, Color)> {
    separated_pair(
        dec_uint,
        ' ',
        alt((
            "red".value(Color::Red),
            "green".value(Color::Green),
            "blue".value(Color::Blue),
        )),
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
        Day::test_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_b()
    }
}
