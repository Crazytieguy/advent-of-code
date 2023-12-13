use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<Vec<&'static str>>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 405;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 400;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        Ok(input
            .split("\n\n")
            .map(|note| note.lines().collect())
            .collect())
    }

    fn part_a(notes: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        notes.iter().map(|note| score(note, 0)).sum()
    }

    fn part_b(notes: Self::Shared) -> anyhow::Result<Self::Answer> {
        notes.iter().map(|note| score(note, 1)).sum()
    }
}

fn score(note: &[&str], allowed_mismatches: usize) -> anyhow::Result<usize> {
    if let Some(mirror_column) =
        find_reflection_index(note.iter().map(|row| row.bytes()), allowed_mismatches)
    {
        Ok(mirror_column)
    } else if let Some(mirror_row) = find_reflection_index(
        (0..note[0].len()).map(|i| note.iter().map(move |row| row.as_bytes()[i])),
        allowed_mismatches,
    ) {
        Ok(mirror_row * 100)
    } else {
        Err(anyhow!("No mirror found for note\n{note:?}"))
    }
}

fn find_reflection_index<T: DoubleEndedIterator<Item = u8> + ExactSizeIterator + Clone>(
    note: impl Iterator<Item = T> + Clone,
    allowed_mismatches: usize,
) -> Option<usize> {
    (1..note.clone().next()?.len()).find(|&i| {
        note.clone()
            .map(|row| {
                row.clone()
                    .take(i)
                    .rev()
                    .zip(row.skip(i))
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .sum::<usize>()
            == allowed_mismatches
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
