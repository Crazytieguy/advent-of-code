#![warn(clippy::pedantic)]
use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use anyhow::bail;
use winnow::{
    ascii::{dec_uint, space1},
    combinator::{opt, separated},
    seq, Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct Records {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Records;
    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 288;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 71503;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        records.parse(input).map_err(anyhow::Error::msg)
    }

    fn part_a(records: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        if records.times.len() != records.distances.len() {
            bail!(
                "times and distances have different lenghts.
times={:?}
distances={:?}",
                records.times,
                records.distances
            );
        }
        records
            .times
            .iter()
            .zip(&records.distances)
            .map(|(&time, &distance)| possible_ways_to_win(time, distance))
            .product::<Option<u64>>()
            .ok_or_else(|| anyhow::anyhow!("A cast failed"))
    }

    fn part_b(Records { times, distances }: Self::Shared) -> anyhow::Result<Self::Answer> {
        let time = join_numbers(&times)?;
        let distance = join_numbers(&distances)?;
        possible_ways_to_win(time, distance).ok_or_else(|| anyhow::anyhow!("A cast failed"))
    }
}

fn possible_ways_to_win(time: u64, record_distance: u64) -> Option<u64> {
    // distance = (time - hold_time) * hold_time
    // distance = time * hold_time - hold_time^2
    // hold_time^2 - time * hold_time + distance = 0
    // hold_time = (time +- sqrt(time^2 - 4 * distance)) / 2

    let time: f64 = num::cast(time)?;
    let record_distance: f64 = num::cast(record_distance)?;
    let root_term = time.powi(2) - 4. * record_distance;
    let smallest_hold_time_to_match_record = (time - root_term.sqrt()) / 2.;
    let largest_hold_time_to_match_record = f64::midpoint(time, root_term.sqrt());
    let smallest_int_hold_time_to_beet_record: u64 =
        num::cast(smallest_hold_time_to_match_record.next_up().ceil())?;
    let largest_int_hold_time_to_beet_record: u64 =
        num::cast(largest_hold_time_to_match_record.next_down().floor())?;

    Some(largest_int_hold_time_to_beet_record - smallest_int_hold_time_to_beet_record + 1)
}

fn join_numbers(distances: &[u64]) -> Result<u64, std::num::ParseIntError> {
    distances
        .iter()
        .map(ToString::to_string)
        .collect::<String>()
        .parse()
}

fn records(input: &mut &'static str) -> winnow::PResult<Records> {
    seq! {Records{
        _: ("Time:", space1),
        times: numbers,
        _: ("\nDistance:", space1),
        distances: numbers,
        _: opt("\n"),
    }}
    .parse_next(input)
}

fn numbers(input: &mut &'static str) -> winnow::PResult<Vec<u64>> {
    separated(1.., dec_uint::<_, u64, _>, space1).parse_next(input)
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
