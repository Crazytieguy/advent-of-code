#![feature(float_next_up_down)]
use advent_2023::{BasicSolution, Solution};
use anyhow::bail;
use winnow::{
    ascii::{dec_uint, space1},
    combinator::{opt, separated},
    Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct Records {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Records;
    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 288;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 71503;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        records.parse(data).map_err(anyhow::Error::msg)
    }

    fn part_a(Records { times, distances }: Self::Parsed) -> anyhow::Result<Self::Answer> {
        if times.len() != distances.len() {
            bail!(
                "times and distances have different lenghts.
times={times:?}
distances={distances:?}"
            );
        }
        Ok(times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| possible_ways_to_win(time, distance))
            .product())
    }

    fn part_b(Records { times, distances }: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let time = times
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .parse::<u64>()?;
        let distance = distances
            .iter()
            .map(ToString::to_string)
            .collect::<String>()
            .parse::<u64>()?;
        Ok(possible_ways_to_win(time, distance))
    }
}

fn possible_ways_to_win(time: u64, record_distance: u64) -> u64 {
    // distance = (time - hold_time) * hold_time
    // distance = time * hold_time - hold_time^2
    // hold_time^2 - time * hold_time + distance = 0
    // hold_time = (time +- sqrt(time^2 - 4 * distance)) / 2

    let time = time as f64;
    let record_distance = record_distance as f64;
    let root_term = time.powi(2) - 4. * record_distance;
    let smallest_hold_time_to_match_record = (time - root_term.sqrt()) / 2.;
    let largest_hold_time_to_match_record = (time + root_term.sqrt()) / 2.;
    let smallest_int_hold_time_to_beet_record =
        smallest_hold_time_to_match_record.next_up().ceil() as u64;
    let largest_int_hold_time_to_beet_record =
        largest_hold_time_to_match_record.next_down().floor() as u64;

    largest_int_hold_time_to_beet_record - smallest_int_hold_time_to_beet_record + 1
}

fn records(data: &mut &'static str) -> winnow::PResult<Records> {
    (
        ("Time:", space1),
        numbers,
        ("\n", "Distance:", space1),
        numbers,
        opt("\n"),
    )
        .map(|(_, times, _, distances, _)| Records { times, distances })
        .parse_next(data)
}

fn numbers(data: &mut &'static str) -> winnow::PResult<Vec<u64>> {
    separated(1.., dec_uint::<_, u64, _>, space1).parse_next(data)
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
