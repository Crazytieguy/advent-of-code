use advent_2022::*;
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
use std::{cmp::Reverse, collections::VecDeque};

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<Monkey>;
    type A = u64;
    type B = u64;
    const SAMPLE_ANSWER_A: Self::TestA = 10605;
    const SAMPLE_ANSWER_B: Self::TestB = 2713310158;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, monkey)(data)
    }

    fn a(mut monkeys: Self::Parsed) -> Self::A {
        stuff_slinging_simian_shenanigans(&mut monkeys, 20, |n| n / 3)
    }

    fn b(mut monkeys: Self::Parsed) -> Self::B {
        let least_common_denominator: u64 = monkeys.iter().map(|m| m.test).product();
        stuff_slinging_simian_shenanigans(&mut monkeys, 10000, |n| n % least_common_denominator)
    }
}

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

fn stuff_slinging_simian_shenanigans(
    monkeys: &mut [Monkey],
    rounds: usize,
    manage_worry_level: impl Fn(u64) -> u64,
) -> u64 {
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
    monkey_business(monkeys)
}

fn monkey_business(monkeys: &[Monkey]) -> u64 {
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted_by_key(|&n| Reverse(n))
        .take(2)
        .product()
}
