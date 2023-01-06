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

fn parse(data: &str) -> IResult<i64> {
    fold_many1(snafu.terminated(line_ending), || 0, |acc, cur| acc + cur)(data)
}

fn to_snafu(number: i64) -> String {
    itertools::unfold(number, |number| {
        if *number == 0 {
            return None;
        }
        let digit_value = (((*number % 5) + 2) % 5) - 2;
        *number -= digit_value;
        *number /= 5;
        Some(match digit_value {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        })
    })
    .collect::<String>()
    .chars()
    .rev()
    .collect()
}

fn solution(data: i64) -> String {
    to_snafu(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test() -> OutResult {
        assert_eq!(solution(parse(SAMPLE_DATA)?.1), "2=-1=0");
        println!("solution: {}", solution(parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("solution: {}", solution(parsed));
    Ok(())
}
