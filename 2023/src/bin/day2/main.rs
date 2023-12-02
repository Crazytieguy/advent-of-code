use advent_2023::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u32, u8},
    multi::{fold_many_m_n, separated_list1},
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;

struct Day;

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
                game.subsets
                    .iter()
                    .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
            })
            .map(|game| game.id)
            .sum())
    }

    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(data
            .into_iter()
            .map(|game| {
                let max_subset =
                    game.subsets
                        .iter()
                        .fold(Subset::default(), |mut max_subset, cur_subset| {
                            max_subset.red = max_subset.red.max(cur_subset.red);
                            max_subset.green = max_subset.green.max(cur_subset.green);
                            max_subset.blue = max_subset.blue.max(cur_subset.blue);
                            max_subset
                        });
                max_subset.red as u32 * max_subset.green as u32 * max_subset.blue as u32
            })
            .sum())
    }
}

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

fn game(data: &str) -> IResult<Game> {
    let (data, id) = tag("Game ")
        .precedes(u32)
        .terminated(tag(": "))
        .parse(data)?;
    let (data, subsets) = separated_list1(tag("; "), subset)(data)?;
    Ok((data, Game { id, subsets }))
}

#[derive(Debug, Clone, Default)]
struct Subset {
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

fn cubes(data: &str) -> IResult<(u8, Color)> {
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

fn subset(data: &str) -> IResult<Subset> {
    fold_many_m_n(
        1,
        3,
        cubes.terminated(tag(", ").opt()),
        Subset::default,
        |mut acc, (n, color)| {
            match color {
                Color::Red => acc.red = n,
                Color::Green => acc.green = n,
                Color::Blue => acc.blue = n,
            }
            acc
        },
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
