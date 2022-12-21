use std::{collections::HashMap, error::Error};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    sequence::separated_pair,
    Parser,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed<'a> = HashMap<&'a str, Expression<'a>>;

#[derive(Debug, Clone, Copy)]
enum Expression<'a> {
    Number(i64),
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
}

fn parse_expression(input: &str) -> IResult<Expression> {
    alt((
        i64.map(Expression::Number),
        separated_pair(alpha1, tag(" + "), alpha1).map(|(a, b)| Expression::Add(a, b)),
        separated_pair(alpha1, tag(" - "), alpha1).map(|(a, b)| Expression::Subtract(a, b)),
        separated_pair(alpha1, tag(" * "), alpha1).map(|(a, b)| Expression::Multiply(a, b)),
        separated_pair(alpha1, tag(" / "), alpha1).map(|(a, b)| Expression::Divide(a, b)),
    ))(input)
}

fn parse_monkey(input: &str) -> IResult<(&str, Expression)> {
    separated_pair(alpha1, tag(": "), parse_expression)(input)
}

fn parse(data: &str) -> IResult<Parsed> {
    let mut monkeys = HashMap::new();
    for line in data.lines() {
        let (_, (monkey, expr)) = parse_monkey(line)?;
        monkeys.insert(monkey, expr);
    }
    Ok(("", monkeys))
}

fn eval<'a>(monkey: &'a str, monkeys: &'a Parsed, memo: &mut HashMap<&'a str, i64>) -> i64 {
    if let Some(&val) = memo.get(monkey) {
        return val;
    }
    let val = match monkeys[monkey] {
        Expression::Number(n) => n,
        Expression::Add(a, b) => eval(a, monkeys, memo) + eval(b, monkeys, memo),
        Expression::Subtract(a, b) => eval(a, monkeys, memo) - eval(b, monkeys, memo),
        Expression::Multiply(a, b) => eval(a, monkeys, memo) * eval(b, monkeys, memo),
        Expression::Divide(a, b) => eval(a, monkeys, memo) / eval(b, monkeys, memo),
    };
    memo.insert(monkey, val);
    val
}

fn part_a(data: &Parsed) -> i64 {
    eval("root", data, &mut HashMap::new())
}

fn eval_b<'a>(
    monkey: &'a str,
    monkeys: &'a Parsed,
    memo: &mut HashMap<&'a str, i64>,
) -> Option<i64> {
    if monkey == "humn" {
        return None;
    }
    let val = match monkeys[monkey] {
        Expression::Number(n) => n,
        Expression::Add(a, b) => eval_b(a, monkeys, memo)
            .zip(eval_b(b, monkeys, memo))
            .map(|(a, b)| a + b)?,
        Expression::Subtract(a, b) => eval_b(a, monkeys, memo)
            .zip(eval_b(b, monkeys, memo))
            .map(|(a, b)| a - b)?,
        Expression::Multiply(a, b) => eval_b(a, monkeys, memo)
            .zip(eval_b(b, monkeys, memo))
            .map(|(a, b)| a * b)?,
        Expression::Divide(a, b) => eval_b(a, monkeys, memo)
            .zip(eval_b(b, monkeys, memo))
            .map(|(a, b)| a / b)?,
    };
    memo.insert(monkey, val);
    Some(val)
}

fn part_b(data: &Parsed) -> i64 {
    let Expression::Add(left, right) = data["root"] else {
        panic!("root is not an add");
    };
    let mut memo = HashMap::new();
    let (mut unknown_should_eq, mut unknown) = match (
        eval_b(left, data, &mut memo),
        eval_b(right, data, &mut memo),
    ) {
        (None, Some(val)) => (val, left),
        (Some(val), None) => (val, right),
        _ => panic!("exactly one side should be known"),
    };
    while unknown != "humn" {
        (unknown_should_eq, unknown) = match data[unknown] {
            Expression::Number(_) => panic!("unknown should not be a number"),
            Expression::Add(left, right) => match (memo.get(&left), memo.get(&right)) {
                (None, Some(val)) => (unknown_should_eq - val, left),
                (Some(val), None) => (unknown_should_eq - val, right),
                _ => panic!("exactly one side should be known"),
            },
            Expression::Subtract(left, right) => match (memo.get(&left), memo.get(&right)) {
                (None, Some(val)) => (unknown_should_eq + val, left),
                (Some(val), None) => (val - unknown_should_eq, right),
                _ => panic!("exactly one side should be known"),
            },
            Expression::Multiply(left, right) => match (memo.get(&left), memo.get(&right)) {
                (None, Some(val)) => (unknown_should_eq / val, left),
                (Some(val), None) => (unknown_should_eq / val, right),
                _ => panic!("exactly one side should be known"),
            },
            Expression::Divide(left, right) => match (memo.get(&left), memo.get(&right)) {
                (None, Some(val)) => (unknown_should_eq * val, left),
                (Some(val), None) => (val / unknown_should_eq, right),
                _ => panic!("exactly one side should be known"),
            },
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
    let start = std::time::Instant::now();
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    println!("time: {:?}", start.elapsed());
    Ok(())
}
