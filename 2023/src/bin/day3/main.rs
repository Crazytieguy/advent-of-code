use std::collections::HashMap;

use advent_2023::*;
use itertools::Itertools;

struct Day;

#[derive(Debug, Clone)]
struct Schematic<'a> {
    raw: Vec<&'a [u8]>,
    numbers: Vec<Number>,
}

#[derive(Debug, Clone)]
struct Number {
    row: usize,
    start_column: usize,
    end_column: usize,
    value: u32,
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Schematic<'static>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 4361;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 467835;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        let numbers = data
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(col, c)| {
                        c.to_digit(10).map(|value| Number {
                            row,
                            start_column: col,
                            end_column: col,
                            value,
                        })
                    })
                    .coalesce(|first, second| {
                        if first.end_column + 1 == second.start_column {
                            Ok(Number {
                                row: first.row,
                                start_column: first.start_column,
                                end_column: second.end_column,
                                value: first.value * 10 + second.value,
                            })
                        } else {
                            Err((first, second))
                        }
                    })
            })
            .collect();
        let raw = data.lines().map(str::as_bytes).collect_vec();
        Ok(("", Schematic { raw, numbers }))
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

impl Number {
    fn adjacent_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let rows_to_check = self.row.saturating_sub(1)..=self.row + 1;
        let columns_to_check = self.start_column.saturating_sub(1)..=self.end_column + 1;
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
