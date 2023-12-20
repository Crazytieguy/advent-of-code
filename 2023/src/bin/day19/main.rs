use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Index, IndexMut, RangeInclusive},
};

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
struct PartRanges {
    x: RangeInclusive<u16>,
    m: RangeInclusive<u16>,
    a: RangeInclusive<u16>,
    s: RangeInclusive<u16>,
}

#[derive(Debug, Clone)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
enum Category {
    x,
    m,
    a,
    s,
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
                        Destination::Accept => {
                            return part.x as usize
                                + part.m as usize
                                + part.a as usize
                                + part.s as usize
                        }
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
            PartRanges {
                x: 1..=4000,
                m: 1..=4000,
                a: 1..=4000,
                s: 1..=4000,
            },
        ))
    }
}

fn distinct_combinations(
    workflows: &HashMap<&'static str, WorkFlow<'static>>,
    destination: Destination<'static>,
    mut part_ranges: PartRanges,
) -> usize {
    if part_ranges.x.is_empty()
        || part_ranges.m.is_empty()
        || part_ranges.a.is_empty()
        || part_ranges.s.is_empty()
    {
        return 0;
    }
    let workflow = match destination {
        Destination::Accept => {
            return part_ranges.x.len()
                * part_ranges.m.len()
                * part_ranges.a.len()
                * part_ranges.s.len()
        }
        Destination::Reject => return 0,
        Destination::Workflow(w) => w,
    };
    let mut count = 0;
    for rule in &workflows[workflow].rules {
        let SplitPartRanges { pass, not_pass } = part_ranges.split(rule.condition);
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

struct SplitPartRanges {
    pass: PartRanges,
    not_pass: PartRanges,
}

impl PartRanges {
    fn split(&self, condition: Condition) -> SplitPartRanges {
        let mut pass = self.clone();
        let mut not_pass = self.clone();
        match condition.operator {
            Operator::GreaterThan => {
                pass[condition.category] = (*pass[condition.category].start())
                    .max(condition.value + 1)
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
}

impl IndexMut<Category> for PartRanges {
    fn index_mut(&mut self, index: Category) -> &mut Self::Output {
        match index {
            Category::x => &mut self.x,
            Category::m => &mut self.m,
            Category::a => &mut self.a,
            Category::s => &mut self.s,
        }
    }
}

impl Index<Category> for PartRanges {
    type Output = RangeInclusive<u16>;

    fn index(&self, index: Category) -> &Self::Output {
        match index {
            Category::x => &self.x,
            Category::m => &self.m,
            Category::a => &self.a,
            Category::s => &self.s,
        }
    }
}

impl IndexMut<Category> for Part {
    fn index_mut(&mut self, index: Category) -> &mut Self::Output {
        match index {
            Category::x => &mut self.x,
            Category::m => &mut self.m,
            Category::a => &mut self.a,
            Category::s => &mut self.s,
        }
    }
}

impl Index<Category> for Part {
    type Output = u16;

    fn index(&self, index: Category) -> &Self::Output {
        match index {
            Category::x => &self.x,
            Category::m => &self.m,
            Category::a => &self.a,
            Category::s => &self.s,
        }
    }
}

fn workflow(input: &mut &'static str) -> winnow::PResult<WorkFlow<'static>> {
    seq! { WorkFlow {
        _: '{',
        rules: separated(1.., rule, ','),
        _: ',',
        fallback: destination,
        _: '}',
    }}
    .parse_next(input)
}

fn part(input: &mut &'static str) -> winnow::PResult<Part> {
    seq! { Part {
        _: "{x=",
        x: dec_uint,
        _: ",m=",
        m: dec_uint,
        _: ",a=",
        a: dec_uint,
        _: ",s=",
        s: dec_uint,
        _: '}',
    }}
    .parse_next(input)
}

fn rule(input: &mut &'static str) -> winnow::PResult<Rule<'static>> {
    seq! {Rule {
        condition: condition,
        _: ':',
        destination: destination,
    }}
    .parse_next(input)
}

fn destination(input: &mut &'static str) -> winnow::PResult<Destination<'static>> {
    alt((
        'A'.value(Destination::Accept),
        'R'.value(Destination::Reject),
        alpha1.map(Destination::Workflow),
    ))
    .parse_next(input)
}

fn condition(input: &mut &'static str) -> winnow::PResult<Condition> {
    seq! {Condition {
        category: category,
        operator: operator,
        value: dec_uint,
    }}
    .parse_next(input)
}

fn operator(input: &mut &'static str) -> winnow::PResult<Operator> {
    alt((
        '>'.value(Operator::GreaterThan),
        '<'.value(Operator::LessThan),
    ))
    .parse_next(input)
}

fn category(input: &mut &'static str) -> winnow::PResult<Category> {
    alt((
        'x'.value(Category::x),
        'm'.value(Category::m),
        'a'.value(Category::a),
        's'.value(Category::s),
    ))
    .parse_next(input)
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
