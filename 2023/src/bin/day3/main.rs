use std::collections::HashMap;

use advent_2023::*;
use itertools::Itertools;

struct Day;

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Schematic;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 4361;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 467835;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        let mut symbols = HashMap::new();
        let mut numbers = Vec::new();
        data.lines().enumerate().for_each(|(row, line)| {
            let mut current_number: Option<Number> = None;
            line.chars()
                .enumerate()
                .for_each(|(column, c)| match c.to_digit(10) {
                    Some(digit) => {
                        current_number = match current_number.take() {
                            Some(mut number) => {
                                number.end_column = column;
                                number.value = number.value * 10 + digit;
                                Some(number)
                            }
                            None => Some(Number {
                                row,
                                start_column: column,
                                end_column: column,
                                value: digit,
                            }),
                        };
                    }
                    None => {
                        if let Some(number) = current_number.take() {
                            numbers.push(number);
                        }
                        if c != '.' {
                            symbols.insert((row, column), c);
                        }
                    }
                });
            if let Some(number) = current_number {
                numbers.push(number);
            }
        });
        Ok(("", Schematic { symbols, numbers }))
    }

    fn a(Schematic { numbers, symbols }: Self::Parsed) -> anyhow::Result<Self::Answer> {
        Ok(numbers
            .into_iter()
            .filter(|number| {
                number
                    .adjacent_coords()
                    .any(|coords| symbols.contains_key(&coords))
            })
            .map(|number| number.value)
            .sum())
    }

    fn b(Schematic { numbers, symbols }: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let mut potential_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        numbers.into_iter().for_each(|number| {
            number
                .adjacent_coords()
                .filter(|coords| symbols.get(coords) == Some(&'*'))
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

#[derive(Debug, Clone)]
struct Number {
    row: usize,
    start_column: usize,
    end_column: usize,
    value: u32,
}

impl Number {
    fn adjacent_coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let rows_to_check = self.row.saturating_sub(1)..=self.row + 1;
        let columns_to_check = self.start_column.saturating_sub(1)..=self.end_column + 1;
        rows_to_check.cartesian_product(columns_to_check)
    }
}

#[derive(Debug, Clone)]
struct Schematic {
    symbols: HashMap<(usize, usize), char>,
    numbers: Vec<Number>,
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
