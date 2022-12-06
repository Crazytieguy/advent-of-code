use std::cmp::Reverse;
use std::error::Error;

use itertools::Itertools;
use nom::character::complete::{line_ending, u32};
use nom::multi::{fold_many1, separated_list1};
use nom_supreme::ParserExt;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = Vec<usize>;

fn parse(data: &'static str) -> IResult<Parsed> {
    separated_list1(
        line_ending,
        fold_many1(
            u32.terminated(line_ending),
            || 0,
            |total, item| total + item as usize,
        ),
    )(data)
}

fn part_a(data: &Parsed) -> usize {
    data.iter().copied().max().expect("At least one elf")
}

fn part_b(data: &Parsed) -> usize {
    data.iter()
        .sorted_unstable_by_key(|&cals| Reverse(cals))
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 24000);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 45000);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
