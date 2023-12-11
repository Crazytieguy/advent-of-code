#![warn(clippy::pedantic)]
use std::{borrow::Cow, collections::HashMap, ops::Range};

use advent_2023::{BasicSolution, Solution};
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
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Schematic<'static>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 4361;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 467_835;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        let mut numbers = Vec::new();
        for (row, line) in input.lines().enumerate() {
            let mut iter_nums =
                iterator(Located::new(line), preceded(not_dec, dec_uint.with_span()));
            numbers.extend(iter_nums.map(|(value, columns)| Number {
                row,
                columns,
                value,
            }));
            // Purely for error checking
            let (rest, ()) = iter_nums.finish().map_err(anyhow::Error::msg)?;
            not_dec.parse(rest).map_err(anyhow::Error::msg)?;
        }
        let raw = input.lines().map(str::as_bytes).collect_vec();
        Ok(Schematic { raw, numbers })
    }

    fn part_a(schematic: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(schematic
            .numbers
            .iter()
            .filter(|number| {
                number.adjacent_coords().any(|coords| {
                    get_2d(&schematic.raw, coords).is_some_and(|c| !c.is_ascii_digit() && c != b'.')
                })
            })
            .map(|number| number.value)
            .sum())
    }

    fn part_b(Schematic { numbers, raw }: Self::Shared) -> anyhow::Result<Self::Answer> {
        let mut potential_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        for num in numbers {
            num.adjacent_coords()
                .filter(|&coords| get_2d(&raw, coords) == Some(b'*'))
                .for_each(|coords| {
                    potential_gears.entry(coords).or_default().push(num.value);
                });
        }
        Ok(potential_gears
            .into_values()
            .filter(|values| values.len() == 2)
            .map(|values| values.into_iter().product::<u32>())
            .sum())
    }
}

fn not_dec(input: &mut Located<&'static str>) -> winnow::PResult<&'static str> {
    take_till0(AsChar::is_dec_digit).parse_next(input)
}

impl Number {
    fn adjacent_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let rows_to_check = self.row.saturating_sub(1)..=self.row + 1;
        let columns_to_check = self.columns.start.saturating_sub(1)..=self.columns.end;
        rows_to_check.cartesian_product(columns_to_check)
    }
}

fn get_2d(grid: &[&[u8]], (row, col): (usize, usize)) -> Option<u8> {
    grid.get(row).and_then(|line| line.get(col)).copied()
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
