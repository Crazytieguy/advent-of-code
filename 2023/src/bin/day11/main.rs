use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use winnow::{
    combinator::{alt, repeat},
    Parser,
};

struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Galaxy,
    Space,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<Vec<Pixel>>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 374;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 82000210;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        input
            .lines()
            .map(|line| repeat(1.., pixel).parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(image: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(solve::<2>(&image))
    }

    fn part_b(image: Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(solve::<1_000_000>(&image))
    }
}

fn solve<const EXPANSION: usize>(image: &[Vec<Pixel>]) -> usize {
    let total_galaxies = count_galaxies(image.iter().flatten().copied());
    let galaxies_in_each_row = image.iter().map(|row| count_galaxies(row.iter().copied()));
    let total_row_traversals =
        count_traversals_simultaneously::<EXPANSION>(galaxies_in_each_row, total_galaxies);

    let galaxies_in_each_column =
        (0..image[0].len()).map(|col| count_galaxies(image.iter().map(|row| row[col])));
    let total_column_traversals =
        count_traversals_simultaneously::<EXPANSION>(galaxies_in_each_column, total_galaxies);

    total_row_traversals + total_column_traversals
}

fn count_traversals_simultaneously<const EXPANSION: usize>(
    galaxies_in_each_line: impl Iterator<Item = usize>,
    total_galaxies: usize,
) -> usize {
    galaxies_in_each_line
        .fold(
            (0, total_galaxies, 0),
            |(galaxies_behind, galaxies_ahead, traversed), galaxies_in_line| {
                let coefficient = if galaxies_in_line == 0 { EXPANSION } else { 1 };
                (
                    galaxies_behind + galaxies_in_line,
                    galaxies_ahead - galaxies_in_line,
                    traversed + galaxies_behind * galaxies_ahead * coefficient,
                )
            },
        )
        .2
}

fn count_galaxies(pixels: impl Iterator<Item = Pixel>) -> usize {
    pixels.filter(|&p| p == Pixel::Galaxy).count()
}

fn pixel(input: &mut &'static str) -> winnow::PResult<Pixel> {
    alt(('#'.value(Pixel::Galaxy), '.'.value(Pixel::Space))).parse_next(input)
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
