use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use winnow::{
    ascii::dec_uint,
    combinator::{alt, repeat, separated, separated_pair},
    Parser,
};

struct Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringCondition {
    Operational,
    Damaged,
    Unknown,
}

use SpringCondition::{Damaged, Operational, Unknown};

#[derive(Debug, Clone)]
struct ConditionRecord {
    spring_conditions: Vec<SpringCondition>,
    damaged_group_sizes: Vec<usize>,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<ConditionRecord>;
    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 21;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 525152;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        input
            .lines()
            .map(|line| condition_record.parse(line).map_err(anyhow::Error::msg))
            .collect()
    }

    fn part_a(condition_records: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(condition_records
            .iter()
            .cloned()
            .map(|record| record.count_possible_arangements())
            .sum())
    }

    fn part_b(condition_records: Self::Shared) -> anyhow::Result<Self::Answer> {
        Ok(condition_records
            .into_iter()
            .map(|mut record| {
                record.spring_conditions = record
                    .spring_conditions
                    .iter()
                    .copied()
                    .chain([Unknown])
                    .cycle()
                    .take(record.spring_conditions.len() * 5 + 4)
                    .collect();
                record.damaged_group_sizes = record
                    .damaged_group_sizes
                    .iter()
                    .copied()
                    .cycle()
                    .take(record.damaged_group_sizes.len() * 5)
                    .collect();
                record.count_possible_arangements()
            })
            .sum())
    }
}

impl ConditionRecord {
    fn count_possible_arangements(mut self) -> u64 {
        // to make the Damaged recursion case simpler
        self.spring_conditions.push(Operational);
        let mut cache =
            vec![vec![None; self.spring_conditions.len() + 1]; self.damaged_group_sizes.len() + 1];
        count_possible_arangements_inner(
            &self.spring_conditions,
            &self.damaged_group_sizes,
            &mut cache,
        )
    }
}

fn count_possible_arangements_inner(
    spring_conditions: &[SpringCondition],
    damaged_group_sizes: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if let Some(cached) = cache[damaged_group_sizes.len()][spring_conditions.len()] {
        return cached;
    }
    let mut arangements = None;
    if damaged_group_sizes.is_empty() {
        arangements = Some(if spring_conditions.contains(&Damaged) {
            // Too many previous unknowns were counted as damaged
            0
        } else {
            // All remaining unknowns are operational
            1
        });
    }
    if spring_conditions.len()
        < damaged_group_sizes.iter().sum::<usize>() + damaged_group_sizes.len()
    {
        // Not enough space for remaining numbers
        arangements = Some(0);
    }
    if let Some(arangements) = arangements {
        cache[damaged_group_sizes.len()][spring_conditions.len()] = Some(arangements);
        return arangements;
    }
    let mut arangements = 0;
    if spring_conditions[0] != Damaged {
        // Assume operational
        arangements +=
            count_possible_arangements_inner(&spring_conditions[1..], damaged_group_sizes, cache);
    }
    let next_group_size = damaged_group_sizes[0];
    if !spring_conditions[..next_group_size].contains(&Operational)
        && spring_conditions[next_group_size] != Damaged
    {
        // Assume damaged
        arangements += count_possible_arangements_inner(
            &spring_conditions[next_group_size + 1..],
            &damaged_group_sizes[1..],
            cache,
        );
    }
    cache[damaged_group_sizes.len()][spring_conditions.len()] = Some(arangements);
    arangements
}

fn condition_record(input: &mut &'static str) -> winnow::PResult<ConditionRecord> {
    separated_pair(
        repeat(
            1..,
            alt((
                '.'.value(Operational),
                '#'.value(Damaged),
                '?'.value(Unknown),
            )),
        ),
        ' ',
        separated(1.., dec_uint::<_, u16, _>.map(usize::from), ','),
    )
    .map(|(spring_conditions, damaged_group_sizes)| ConditionRecord {
        spring_conditions,
        damaged_group_sizes,
    })
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
