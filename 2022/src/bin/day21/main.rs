use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64, line_ending},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Parser,
};
use nom_supreme::ParserExt;
use std::{collections::HashMap, error::Error};
use Expression::*;
use Operator::*;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed<'a> = HashMap<&'a str, Expression<'a>>;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn eval(self, left: i64, right: i64) -> i64 {
        match self {
            Add => left + right,
            Subtract => left - right,
            Multiply => left * right,
            Divide => left / right,
        }
    }
}

fn operator(input: &str) -> IResult<Operator> {
    alt((
        tag(" + ").value(Add),
        tag(" - ").value(Subtract),
        tag(" * ").value(Multiply),
        tag(" / ").value(Divide),
    ))(input)
}

#[derive(Debug, Clone, Copy)]
enum Expression<'a> {
    Number(i64),
    Operation((&'a str, Operator, &'a str)),
}

fn expression(input: &str) -> IResult<Expression> {
    alt((
        i64.map(Number),
        tuple((alpha1, operator, alpha1)).map(Operation),
    ))(input)
}

fn monkey(input: &str) -> IResult<(&str, Expression)> {
    separated_pair(alpha1, tag(": "), expression)(input)
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, monkey)
        .map(|monkeys| monkeys.into_iter().collect())
        .parse(data)
}

fn eval(monkeys: &Parsed, monkey: &str) -> i64 {
    match monkeys[monkey] {
        Number(n) => n,
        Operation((left_monkey, op, right_monkey)) => {
            op.eval(eval(monkeys, left_monkey), eval(monkeys, right_monkey))
        }
    }
}

fn part_a(data: &Parsed) -> i64 {
    eval(data, "root")
}

impl Operator {
    fn solve_for_left(self, result: i64, right: i64) -> i64 {
        match self {
            Add => result - right,
            Subtract => result + right,
            Multiply => result / right,
            Divide => result * right,
        }
    }

    fn solve_for_right(self, result: i64, left: i64) -> i64 {
        match self {
            Add => result - left,
            Subtract => left - result,
            Multiply => result / left,
            Divide => left / result,
        }
    }
}

fn fill_knowns<'a>(
    knowns: &mut HashMap<&'a str, i64>,
    monkeys: &'a Parsed,
    monkey: &'a str,
) -> Option<i64> {
    if monkey == "humn" {
        return None;
    }
    let val = match monkeys[monkey] {
        Number(n) => n,
        Operation((left_monkey, op, right_monkey)) => {
            let left = fill_knowns(knowns, monkeys, left_monkey);
            let right = fill_knowns(knowns, monkeys, right_monkey);
            op.eval(left?, right?)
        }
    };
    knowns.insert(monkey, val);
    Some(val)
}

fn part_b(data: &Parsed) -> i64 {
    let mut knowns = HashMap::new();
    fill_knowns(&mut knowns, data, "root");

    let (mut unknown, mut unknown_should_eq) = ("root", 0);
    while unknown != "humn" {
        (unknown, unknown_should_eq) = match data[unknown] {
            Operation((left, op, right)) => match (knowns.get(&left), knowns.get(&right)) {
                (None, Some(&val)) if unknown == "root" => (left, val),
                (Some(&val), None) if unknown == "root" => (right, val),
                (None, Some(&val)) => (left, op.solve_for_left(unknown_should_eq, val)),
                (Some(&val), None) => (right, op.solve_for_right(unknown_should_eq, val)),
                _ => panic!("exactly one side of {unknown} should be known"),
            },
            Number(_) => panic!("{unknown} should not be a number"),
        }
    }
    unknown_should_eq
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 152);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 301);
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
