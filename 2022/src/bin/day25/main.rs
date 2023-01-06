use std::error::Error;

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    multi::fold_many1,
};
use nom_supreme::ParserExt;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = i64;

fn snafu_digit(input: &str) -> IResult<i64> {
    alt((
        char('2').value(2),
        char('1').value(1),
        char('0').value(0),
        char('-').value(-1),
        char('=').value(-2),
    ))(input)
}

fn snafu(input: &str) -> IResult<i64> {
    fold_many1(snafu_digit, || 0, |acc, cur| acc * 5 + cur)(input)
}

fn parse(data: &str) -> IResult<Parsed> {
    fold_many1(snafu.terminated(line_ending), || 0, |acc, cur| acc + cur)(data)
}

fn to_snafu(mut number: i64) -> String {
    let mut result = String::new();
    while number != 0 {
        result.push(match number % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                number += 5;
                '='
            }
            4 => {
                number += 5;
                '-'
            }
            _ => unreachable!(),
        });
        number /= 5;
    }
    result.chars().rev().collect()
}

fn solution(data: &Parsed) -> String {
    to_snafu(*data)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test() -> OutResult {
        assert_eq!(solution(&parse(SAMPLE_DATA)?.1), "2=-1=0");
        println!("solution: {}", solution(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("solution: {}", solution(&parsed));
    Ok(())
}
