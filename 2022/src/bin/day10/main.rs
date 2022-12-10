use std::error::Error;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::line_ending, multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(i64),
    Noop,
}

fn parse_operation(input: &str) -> IResult<Operation> {
    alt((
        tag("addx ")
            .precedes(nom::character::complete::i64)
            .map(Operation::Add),
        tag("noop").value(Operation::Noop),
    ))
    .parse(input)
}

type Parsed = Vec<Operation>;

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, parse_operation).parse(data)
}

fn iter_cycle_reg_x(data: &Parsed) -> impl Iterator<Item = (usize, i64)> + '_ {
    data.iter()
        .scan(1, |reg_x, op| {
            Some(match op {
                Operation::Add(x) => {
                    *reg_x += x;
                    [Some(*reg_x - x), Some(*reg_x - x)]
                }
                Operation::Noop => [Some(*reg_x), None],
            })
        })
        .flatten()
        .flatten()
        .enumerate()
}

fn part_a(data: &Parsed) -> i64 {
    iter_cycle_reg_x(data)
        .map(|(i, reg_x)| (i + 1, reg_x))
        .filter(|(i, _)| [20, 60, 100, 140, 180, 220].contains(i))
        .map(|(i, reg_x)| i as i64 * reg_x)
        .sum()
}

fn part_b(data: &Parsed) -> String {
    iter_cycle_reg_x(data)
        .map(|(i, reg_x)| {
            let horizontal_pos = i as i64 % 40;
            match ((horizontal_pos - reg_x).abs() <= 1, horizontal_pos == 39) {
                (true, true) => "#\n",
                (true, false) => "#",
                (false, true) => ".\n",
                (false, false) => ".",
            }
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
