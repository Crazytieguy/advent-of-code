#![warn(clippy::pedantic)]
use std::{borrow::Cow, ops::Range};

use advent_2023::{BasicSolution, Solution};
use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, not_line_ending},
    combinator::{opt, preceded, separated},
    Parser,
};

struct Day;

#[derive(Debug, Clone)]
struct Mapping {
    source: Range<u64>,
    destination_start: u64,
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    all_mappings: Vec<Vec<Mapping>>,
}

#[derive(Debug, Clone)]
struct MappedRange {
    before: Option<Range<u64>>,
    overlap: Option<Range<u64>>,
    after: Option<Range<u64>>,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Common = Almanac;
    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 35;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 46;

    fn common(input: &'static str) -> anyhow::Result<Self::Common> {
        almanac.parse(input).map_err(anyhow::Error::msg)
    }

    fn part_a(almanac: Cow<Self::Common>) -> anyhow::Result<Self::Answer> {
        almanac
            .seeds
            .iter()
            .map(|&seed| {
                almanac.all_mappings.iter().fold(seed, |acc, mappings| {
                    mappings
                        .iter()
                        .find_map(|mapping| {
                            mapping.source.contains(&acc).then(|| mapping.offset(acc))
                        })
                        .unwrap_or(acc)
                })
            })
            .min()
            .ok_or_else(|| anyhow::Error::msg("no seeds"))
    }

    fn part_b(almanac: Self::Common) -> anyhow::Result<Self::Answer> {
        let seed_ranges = almanac.seeds.into_iter().tuples().map(|(a, b)| a..a + b);
        seed_ranges
            .flat_map(|seed_range| {
                almanac
                    .all_mappings
                    .iter()
                    .fold(vec![seed_range], |acc, mappings| {
                        acc.into_iter()
                            .flat_map(|input_range| multi_map_range(input_range, mappings))
                            .collect()
                    })
            })
            .map(|location_range| location_range.start)
            .min()
            .ok_or_else(|| anyhow::Error::msg("no location ranges"))
    }
}

fn multi_map_range(
    input_range: Range<u64>,
    mappings: &[Mapping],
) -> impl Iterator<Item = Range<u64>> {
    let mut overlaps = vec![];
    let mut leftovers = vec![input_range];
    let mut leftovers_queue = vec![];
    for mapping in mappings {
        for input_range in leftovers.drain(..) {
            let mapped = mapping.map_range(input_range);
            overlaps.extend(mapped.overlap);
            leftovers_queue.extend(mapped.before);
            leftovers_queue.extend(mapped.after);
        }
        std::mem::swap(&mut leftovers, &mut leftovers_queue);
    }
    overlaps.into_iter().chain(leftovers)
}

impl Mapping {
    fn offset(&self, value: u64) -> u64 {
        value + self.destination_start - self.source.start
    }

    fn map_range(&self, range: Range<u64>) -> MappedRange {
        let opt_range = |start, end| Some(start..end).filter(|r| !r.is_empty());
        let before = opt_range(range.start, self.source.start.min(range.end));
        let after = opt_range(self.source.end.max(range.start), range.end);
        let overlap = opt_range(
            range.start.max(self.source.start),
            range.end.min(self.source.end),
        )
        .map(|r| self.offset(r.start)..self.offset(r.end));
        MappedRange {
            before,
            overlap,
            after,
        }
    }
}

fn almanac(input: &mut &'static str) -> winnow::PResult<Almanac> {
    (seeds, "\n\n", separated(7..=7, mappings, "\n\n"), opt("\n"))
        .map(|(seeds, _, all_mappings, _)| Almanac {
            seeds,
            all_mappings,
        })
        .parse_next(input)
}

fn seeds(input: &mut &str) -> winnow::PResult<Vec<u64>> {
    preceded("seeds: ", separated(1.., u64, ' ')).parse_next(input)
}

fn mappings(input: &mut &str) -> winnow::PResult<Vec<Mapping>> {
    preceded((not_line_ending, "\n"), separated(1.., mapping, "\n")).parse_next(input)
}

fn mapping(input: &mut &str) -> winnow::PResult<Mapping> {
    (u64, ' ', u64, ' ', u64)
        .map(|(destination_start, _, source_start, _, len)| {
            let source = source_start..source_start + len;
            Mapping {
                source,
                destination_start,
            }
        })
        .parse_next(input)
}

fn u64(input: &mut &str) -> winnow::PResult<u64> {
    dec_uint.parse_next(input)
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
