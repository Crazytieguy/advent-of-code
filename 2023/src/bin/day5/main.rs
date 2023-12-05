use std::ops::Range;

use advent_2023::{BasicSolution, Solution};
use itertools::Itertools;
use winnow::{
    ascii::{dec_uint, line_ending},
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
    all_mappings: [Vec<Mapping>; 7],
}

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Parsed = Almanac;
    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 35;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 46;

    fn parse(data: &'static str) -> anyhow::Result<Self::Parsed> {
        almanac.parse(data).map_err(anyhow::Error::msg)
    }

    fn part_a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        data.seeds
            .into_iter()
            .map(|seed| {
                data.all_mappings.iter().fold(seed, |acc, mappings| {
                    mappings
                        .iter()
                        .find_map(|mapping| mapping.map(acc))
                        .unwrap_or(acc)
                })
            })
            .min()
            .ok_or_else(|| anyhow::Error::msg("no seeds"))
    }

    fn part_b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        let seed_ranges = data.seeds.into_iter().tuples().map(|(a, b)| a..a + b);
        seed_ranges
            .into_iter()
            .flat_map(|seed_range| {
                data.all_mappings
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
    let mut leftovers_a = vec![input_range];
    let mut leftovers_b = vec![];
    for mapping in mappings {
        for input_range in leftovers_a.drain(..) {
            let [before, overlap, after] = mapping.map_range(input_range);
            overlaps.extend(overlap);
            leftovers_b.extend(before);
            leftovers_b.extend(after);
        }
        std::mem::swap(&mut leftovers_a, &mut leftovers_b);
    }
    overlaps.into_iter().chain(leftovers_a)
}

impl Mapping {
    fn offset(&self, value: u64) -> u64 {
        value + self.destination_start - self.source.start
    }

    fn map(&self, value: u64) -> Option<u64> {
        if self.source.contains(&value) {
            Some(self.offset(value))
        } else {
            None
        }
    }

    fn map_range(&self, range: Range<u64>) -> [Option<Range<u64>>; 3] {
        let opt_range = |start, end| Some(start..end).filter(|r| !r.is_empty());
        let before_mapping = opt_range(range.start, self.source.start.min(range.end));
        let after_mapping = opt_range(self.source.end.max(range.start), range.end);
        let overlap = opt_range(
            range.start.max(self.source.start),
            range.end.min(self.source.end),
        )
        .map(|r| self.offset(r.start)..self.offset(r.end));
        [before_mapping, overlap, after_mapping]
    }
}

fn almanac(data: &mut &str) -> winnow::PResult<Almanac> {
    let seeds = seeds.parse_next(data)?;
    let mappings = |tag| {
        preceded(
            (line_ending, line_ending, tag, line_ending),
            separated(1.., mapping, line_ending),
        )
    };
    let all_mappings = [
        mappings("seed-to-soil map:").parse_next(data)?,
        mappings("soil-to-fertilizer map:").parse_next(data)?,
        mappings("fertilizer-to-water map:").parse_next(data)?,
        mappings("water-to-light map:").parse_next(data)?,
        mappings("light-to-temperature map:").parse_next(data)?,
        mappings("temperature-to-humidity map:").parse_next(data)?,
        mappings("humidity-to-location map:").parse_next(data)?,
    ];
    let _ = opt(line_ending).parse_next(data)?;

    Ok(Almanac {
        seeds,
        all_mappings,
    })
}

fn seeds(data: &mut &str) -> winnow::PResult<Vec<u64>> {
    preceded("seeds: ", separated(1.., dec_uint::<_, u64, _>, ' ')).parse_next(data)
}

fn mapping(data: &mut &str) -> winnow::PResult<Mapping> {
    let (destination_start, _, source_start, _, len): (u64, _, u64, _, u64) =
        (dec_uint, ' ', dec_uint, ' ', dec_uint).parse_next(data)?;
    let source = source_start..source_start + len;
    Ok(Mapping {
        source,
        destination_start,
    })
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
