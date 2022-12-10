use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;
use std::{error::Error, iter};
use Operation::*;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(i64),
    Noop,
}

fn parse_operation(input: &str) -> IResult<Operation> {
    alt((tag("addx ").precedes(i64).map(Add), tag("noop").value(Noop)))(input)
}

type Parsed = Vec<Operation>;

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, parse_operation)(data)
}

fn iter_register(data: &Parsed) -> impl Iterator<Item = i64> + '_ {
    data.iter()
        .scan(1, |register, op| {
            Some(repeat_n(
                *register, // dereference before mutating
                match op {
                    Add(x) => {
                        *register += x;
                        2
                    }
                    Noop => 1,
                },
            ))
        })
        .flatten()
}

fn part_a(data: &Parsed) -> i64 {
    iter_register(data)
        .zip(1..)
        .filter(|(_, cycle)| [20, 60, 100, 140, 180, 220].contains(cycle))
        .map(|(reg_x, cycle)| reg_x * cycle)
        .sum()
}

fn part_b(data: &Parsed) -> String {
    iter_register(data)
        .chunks(40)
        .into_iter()
        .flat_map(|row| {
            row.zip(0..)
                .map(|(x, pos)| if x.abs_diff(pos) <= 1 { '#' } else { '.' })
                .chain(iter::once('\n'))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 13140);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(
            part_b(&parse(SAMPLE_DATA)?.1),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
        println!("part b:\n{}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
