use advent_2023::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u8},
    multi::{fold_many1, separated_list1},
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;

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

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, game)(data)
    }

    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .filter(|game| {
                let Revealed { red, green, blue } = game.revealed;
                red <= 12 && green <= 13 && blue <= 14
            })
            .map(|game| game.id as u32)
            .sum())
    }

    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .map(|game| {
                let Revealed { red, green, blue } = game.revealed;
                red as u32 * green as u32 * blue as u32
            })
            .sum())
    }
}

fn game(data: &str) -> IResult<Game> {
    separated_pair(tag("Game ").precedes(u8), tag(": "), revealed)
        .map(|(id, revealed)| Game { id, revealed })
        .parse(data)
}

fn revealed(data: &str) -> IResult<Revealed> {
    fold_many1(
        color_count.terminated(tag(", ").or(tag("; ")).opt()),
        Revealed::default,
        |mut acc, (n, color)| {
            match color {
                Color::Red => acc.red = acc.red.max(n),
                Color::Green => acc.green = acc.green.max(n),
                Color::Blue => acc.blue = acc.blue.max(n),
            }
            acc
        },
    )(data)
}

fn color_count(data: &str) -> IResult<(u8, Color)> {
    separated_pair(
        u8,
        char(' '),
        alt((
            tag("red").value(Color::Red),
            tag("green").value(Color::Green),
            tag("blue").value(Color::Blue),
        )),
    )(data)
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
