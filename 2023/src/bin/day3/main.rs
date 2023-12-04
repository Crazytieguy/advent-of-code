use std::{collections::HashMap, ops::Range};

use advent_2023::*;
use itertools::Itertools;
use winnow::{
    ascii::dec_uint,
    combinator::{iterator, preceded},
    stream::AsChar,
    token::take_till0,
    Located, Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct Schematic<'a> {
    raw: Vec<&'a [u8]>,
    numbers: Vec<Number>,
}

#[derive(Debug, Clone)]
struct Number {
    row: usize,
    columns: Range<usize>,
    value: u32,
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Schematic<'static>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 4361;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 467835;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        let mut numbers = Vec::new();
        for (row, line) in data.lines().enumerate() {
            let mut iter_nums = iterator(
                Located::new(line),
                preceded(not_number, dec_uint.with_span()),
            );
            numbers.extend(iter_nums.map(|(value, columns)| Number {
                row,
                columns,
                value,
            }));
            // Purely for error checking
            let (rest, _) = iter_nums.finish().map_err(anyhow::Error::msg)?;
            not_number.parse(rest).map_err(anyhow::Error::msg)?;
        }
        let raw = data.lines().map(str::as_bytes).collect_vec();
        Ok(Schematic { raw, numbers })
    }

    fn a(Schematic { numbers, raw }: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(numbers
            .into_iter()
            .filter(|number| {
                number.adjacent_coords().any(|coords| {
                    get_2d(&raw, coords).is_some_and(|c| !c.is_ascii_digit() && c != b'.')
                })
            })
            .map(|number| number.value)
            .sum())
    }

    fn b(Schematic { numbers, raw }: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut potential_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        numbers.into_iter().for_each(|number| {
            number
                .adjacent_coords()
                .filter(|&coords| get_2d(&raw, coords) == Some(b'*'))
                .for_each(|coords| {
                    potential_gears
                        .entry(coords)
                        .or_default()
                        .push(number.value);
                });
        });
        Ok(potential_gears
            .into_values()
            .filter(|values| values.len() == 2)
            .map(|values| values.into_iter().product::<u32>())
            .sum())
    }
}

fn not_number(data: &mut Located<&'static str>) -> winnow::PResult<&'static str> {
    take_till0(AsChar::is_dec_digit).parse_next(data)
}

impl Number {
    fn adjacent_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let rows_to_check = self.row.saturating_sub(1)..=self.row + 1;
        let columns_to_check = self.columns.start.saturating_sub(1)..self.columns.end + 1;
        rows_to_check.cartesian_product(columns_to_check)
    }
}

fn get_2d(data: &[&[u8]], (row, col): (usize, usize)) -> Option<u8> {
    data.get(row).and_then(|line| line.get(col)).copied()
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
