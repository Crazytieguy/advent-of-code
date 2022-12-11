use std::{cmp::Reverse, collections::VecDeque, error::Error};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, u64, u8},
    multi::{separated_list0, separated_list1},
    sequence::tuple,
    Parser,
};
use nom_supreme::ParserExt;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = Vec<Monkey>;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    inspected: u64,
    items: VecDeque<u64>,
    operation: Op,
    test: u64,
    if_true: usize,
    if_false: usize,
}

fn monkey_number(data: &str) -> IResult<usize> {
    tag("Monkey ")
        .precedes(u8.map(|n| n as usize))
        .terminated(char(':').and(line_ending))
        .parse(data)
}

fn starting_items(data: &str) -> IResult<VecDeque<u64>> {
    tag("  Starting items: ")
        .precedes(separated_list0(tag(", "), u64).map(Into::into))
        .terminated(line_ending)
        .parse(data)
}

fn operation(data: &str) -> IResult<Op> {
    tag("  Operation: new = old ")
        .precedes(alt((
            tag("+ ").precedes(u64).map(Op::Add),
            tag("* ").precedes(u64).map(Op::Multiply),
            tag("* old").value(Op::Square),
        )))
        .terminated(line_ending)
        .parse(data)
}

fn test(data: &str) -> IResult<u64> {
    tag("  Test: divisible by ")
        .precedes(u64)
        .terminated(line_ending)
        .parse(data)
}

fn if_true(data: &str) -> IResult<usize> {
    tag("    If true: throw to monkey ")
        .precedes(u8.map(|n| n as usize))
        .terminated(line_ending)
        .parse(data)
}

fn if_false(data: &str) -> IResult<usize> {
    tag("    If false: throw to monkey ")
        .precedes(u8.map(|n| n as usize))
        .terminated(line_ending)
        .parse(data)
}

fn monkey(data: &str) -> IResult<Monkey> {
    tuple((
        monkey_number,
        starting_items,
        operation,
        test,
        if_true,
        if_false,
    ))
    .map(|(_, items, operation, test, if_true, if_false)| Monkey {
        inspected: 0,
        items,
        operation,
        test,
        if_true,
        if_false,
    })
    .parse(data)
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, monkey)(data)
}

fn stuff_slinging_simian_shenanigans(
    monkeys: &mut [Monkey],
    rounds: usize,
    manage_worry_level: impl Fn(u64) -> u64,
) {
    for (_, turn) in (0..rounds).cartesian_product(0..monkeys.len()) {
        while let Some(item) = monkeys[turn].items.pop_front() {
            monkeys[turn].inspected += 1;
            let new = match monkeys[turn].operation {
                Op::Add(n) => item + n,
                Op::Multiply(n) => item * n,
                Op::Square => item * item,
            };
            let new = manage_worry_level(new);
            let throw_to = if new % monkeys[turn].test == 0 {
                monkeys[turn].if_true
            } else {
                monkeys[turn].if_false
            };
            monkeys[throw_to].items.push_back(new);
        }
    }
}

fn monkey_business(monkeys: Vec<Monkey>) -> u64 {
    monkeys
        .into_iter()
        .map(|m| m.inspected)
        .sorted_by_key(|&n| Reverse(n))
        .take(2)
        .product()
}

fn part_a(data: &Parsed) -> u64 {
    let mut monkeys = data.clone();
    stuff_slinging_simian_shenanigans(&mut monkeys, 20, |n| n / 3);
    monkey_business(monkeys)
}

fn part_b(data: &Parsed) -> u64 {
    let mut monkeys = data.clone();
    let least_common_denominator: u64 = monkeys.iter().map(|m| m.test).product();
    stuff_slinging_simian_shenanigans(&mut monkeys, 10000, |n| n % least_common_denominator);
    monkey_business(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 10605);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 2713310158);
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
