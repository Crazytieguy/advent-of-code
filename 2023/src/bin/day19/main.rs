use std::{array, borrow::Cow, collections::HashMap, ops::RangeInclusive};

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;
use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::{alt, separated, seq},
    Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct WorkFlow<'a> {
    rules: Vec<Rule<'a>>,
    fallback: Destination<'a>,
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    condition: Condition,
    destination: Destination<'a>,
}

#[derive(Debug, Clone, Copy)]
enum Destination<'a> {
    Accept,
    Reject,
    Workflow(&'a str),
}

#[derive(Debug, Clone, Copy)]
struct Condition {
    category: Category,
    operator: Operator,
    value: u16,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    GreaterThan,
    LessThan,
}

type PartRanges = [RangeInclusive<u16>; 4];
type Part = [u16; 4];
type Category = usize;

struct SplitPartRanges {
    pass: PartRanges,
    not_pass: PartRanges,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = (HashMap<&'static str, WorkFlow<'static>>, Vec<Part>);
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 19114;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 167409079868000;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        let (workflows, parts) = input
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("No double newline"))?;
        let workflows = workflows
            .lines()
            .map(|line| (alpha1, workflow).parse(line).map_err(anyhow::Error::msg))
            .collect::<anyhow::Result<_>>()?;
        let parts = parts
            .lines()
            .map(|line| part.parse(line).map_err(anyhow::Error::msg))
            .collect::<anyhow::Result<_>>()?;
        Ok((workflows, parts))
    }

    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        let (workflows, parts) = shared.as_ref();
        Ok(parts
            .iter()
            .map(|part| {
                let mut workflow = "in";
                loop {
                    match workflows[workflow].run(part) {
                        Destination::Accept => return part.iter().copied().map(usize::from).sum(),
                        Destination::Reject => return 0,
                        Destination::Workflow(next) => workflow = next,
                    }
                }
            })
            .sum())
    }

    fn part_b((workflows, _): Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(distinct_combinations(
            &workflows,
            Destination::Workflow("in"),
            array::from_fn(|_| 1..=4000),
        ))
    }
}

fn distinct_combinations(
    workflows: &HashMap<&'static str, WorkFlow<'static>>,
    destination: Destination<'static>,
    mut part_ranges: PartRanges,
) -> usize {
    if part_ranges.iter().any(|r| r.is_empty()) {
        return 0;
    }
    let workflow = match destination {
        Destination::Accept => return part_ranges.iter().map(|r| r.len()).product(),
        Destination::Reject => return 0,
        Destination::Workflow(w) => w,
    };
    let mut count = 0;
    for rule in &workflows[workflow].rules {
        let SplitPartRanges { pass, not_pass } = split_part_ranges(&part_ranges, rule.condition);
        part_ranges = not_pass;
        count += distinct_combinations(workflows, rule.destination, pass);
    }
    count += distinct_combinations(workflows, workflows[workflow].fallback, part_ranges);
    count
}

impl WorkFlow<'_> {
    fn run(&self, part: &Part) -> Destination<'_> {
        for rule in &self.rules {
            if rule.condition.test(part) {
                return rule.destination;
            }
        }
        self.fallback
    }
}

impl Condition {
    fn test(&self, part: &Part) -> bool {
        match self.operator {
            Operator::GreaterThan => part[self.category] > self.value,
            Operator::LessThan => part[self.category] < self.value,
        }
    }
}

fn split_part_ranges(part_ranges: &PartRanges, condition: Condition) -> SplitPartRanges {
    let mut pass = part_ranges.clone();
    let mut not_pass = part_ranges.clone();
    match condition.operator {
        Operator::GreaterThan => {
            pass[condition.category] = (*pass[condition.category].start()).max(condition.value + 1)
                ..=*pass[condition.category].end();
            not_pass[condition.category] = *not_pass[condition.category].start()
                ..=(*not_pass[condition.category].end()).min(condition.value);
        }
        Operator::LessThan => {
            pass[condition.category] = *pass[condition.category].start()
                ..=(*pass[condition.category].end()).min(condition.value - 1);
            not_pass[condition.category] = (*not_pass[condition.category].start())
                .max(condition.value)
                ..=*not_pass[condition.category].end();
        }
    }
    SplitPartRanges { pass, not_pass }
}

fn workflow(input: &mut &'static str) -> winnow::Result<WorkFlow<'static>> {
    seq! { WorkFlow {
        _: '{',
        rules: separated(1.., rule, ','),
        _: ',',
        fallback: destination,
        _: '}',
    }}
    .parse_next(input)
}

fn part(input: &mut &'static str) -> winnow::Result<Part> {
    seq! { (
        _: "{x=",
        dec_uint,
        _: ",m=",
        dec_uint,
        _: ",a=",
        dec_uint,
        _: ",s=",
        dec_uint,
        _: '}',
    )}
    .parse_next(input)
    .map(From::from)
}

fn rule(input: &mut &'static str) -> winnow::Result<Rule<'static>> {
    seq! {Rule {
        condition: condition,
        _: ':',
        destination: destination,
    }}
    .parse_next(input)
}

fn destination(input: &mut &'static str) -> winnow::Result<Destination<'static>> {
    alt((
        'A'.value(Destination::Accept),
        'R'.value(Destination::Reject),
        alpha1.map(Destination::Workflow),
    ))
    .parse_next(input)
}

fn condition(input: &mut &'static str) -> winnow::Result<Condition> {
    seq! {Condition {
        category: category,
        operator: operator,
        value: dec_uint,
    }}
    .parse_next(input)
}

fn operator(input: &mut &'static str) -> winnow::Result<Operator> {
    alt((
        '>'.value(Operator::GreaterThan),
        '<'.value(Operator::LessThan),
    ))
    .parse_next(input)
}

fn category(input: &mut &'static str) -> winnow::Result<Category> {
    alt(('x'.value(0), 'm'.value(1), 'a'.value(2), 's'.value(3))).parse_next(input)
}

fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_part_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_part_b()
    }
}
