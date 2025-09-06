use std::{array, borrow::Cow};

use advent_2023::{BasicSolution, Solution};
use winnow::{
    ascii::{alpha1, dec_uint},
    combinator::{alt, preceded},
    Parser,
};

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<&'static str>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 1320;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 145;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        Ok(input.trim_end().split(',').collect())
    }

    fn part_a(steps: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        Ok(steps.iter().copied().map(hash).sum())
    }

    fn part_b(steps: Self::Shared) -> anyhow::Result<Self::Answer> {
        let mut hash_map: [Vec<(&str, u8)>; 256] = array::from_fn(|_| Vec::new());
        for step in steps {
            let (label, operation) = (alpha1, operation)
                .parse(step)
                .map_err(anyhow::Error::msg)?;
            let bucket = hash(label);
            match operation {
                Operation::Remove => {
                    hash_map[bucket].retain(|(l, _)| *l != label);
                }
                Operation::Insert(focal_length) => {
                    if let Some((_, current)) =
                        hash_map[bucket].iter_mut().find(|(l, _)| *l == label)
                    {
                        *current = focal_length;
                    } else {
                        hash_map[bucket].push((label, focal_length));
                    }
                }
            }
        }
        Ok(hash_map
            .into_iter()
            .zip(1..)
            .flat_map(|(bucket, box_number)| {
                bucket
                    .into_iter()
                    .zip(1..)
                    .map(move |((_, focal_length), slot_number)| {
                        box_number * slot_number * focal_length as usize
                    })
            })
            .sum())
    }
}

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0, |hash, b| ((hash + b as usize) * 17) % 256)
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Remove,
    Insert(u8),
}

fn operation(input: &mut &'static str) -> winnow::Result<Operation> {
    alt((
        '-'.value(Operation::Remove),
        preceded('=', dec_uint).map(Operation::Insert),
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
