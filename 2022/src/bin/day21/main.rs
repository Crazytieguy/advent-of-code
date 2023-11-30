use std::collections::HashMap;

use advent_2022::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64, line_ending},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    Parser,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = HashMap<&'static str, Expression<'static>>;
    type Answer = i64;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 152;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 301;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, monkey)
            .map(|monkeys| monkeys.into_iter().collect())
            .parse(data)
    }

    fn a(data: Self::Parsed) -> i64 {
        eval(&data, "root")
    }

    fn b(data: Self::Parsed) -> i64 {
        let mut knowns = HashMap::new();
        fill_knowns(&mut knowns, &data, "root");
        let (mut unknown, mut result, mut correction) = ("root", 0, -1);
        while unknown != "humn" {
            let Operation((left, op, right)) = data[unknown] else {
                panic!("{unknown} should be an operation")
            };
            (unknown, result) = match (knowns.get(&left), knowns.get(&right)) {
                (None, Some(&val)) => (left, op.solve_for_left(result, val)),
                (Some(&val), None) => (right, op.solve_for_right(result, val)),
                _ => panic!("exactly one child of {unknown} should be known"),
            };
            result *= correction;
            correction = 1;
        }
        result
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
use Operator::*;

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
use Expression::*;

fn expression(input: &str) -> IResult<Expression> {
    alt((
        i64.map(Number),
        tuple((alpha1, operator, alpha1)).map(Operation),
    ))(input)
}

fn monkey(input: &str) -> IResult<(&str, Expression)> {
    separated_pair(alpha1, tag(": "), expression)(input)
}

fn eval(monkeys: &HashMap<&'static str, Expression<'static>>, monkey: &str) -> i64 {
    match monkeys[monkey] {
        Number(n) => n,
        Operation((left_monkey, op, right_monkey)) => {
            op.eval(eval(monkeys, left_monkey), eval(monkeys, right_monkey))
        }
    }
}

fn fill_knowns<'a>(
    knowns: &mut HashMap<&'a str, i64>,
    monkeys: &'a HashMap<&'static str, Expression<'static>>,
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

impl Operator {
    fn eval(self, left: i64, right: i64) -> i64 {
        match self {
            Add => left + right,
            Subtract => left - right,
            Multiply => left * right,
            Divide => left / right,
        }
    }

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
