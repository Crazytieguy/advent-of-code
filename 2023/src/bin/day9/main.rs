use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;
use itertools::{iterate, Itertools};
use winnow::{ascii::dec_int, combinator::separated, Parser};

struct Day;

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample.txt");

    type Common = Vec<(i32, i32)>;
    type Answer = i32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 114;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 2;

    fn common(data: &'static str) -> anyhow::Result<Self::Common> {
        data.lines()
            .map(|line| {
                history
                    .parse(line)
                    .map_err(anyhow::Error::msg)
                    .and_then(extrapolate)
            })
            .collect()
    }

    fn part_a(data: Self::Common) -> anyhow::Result<Self::Answer> {
        Ok(data.into_iter().map(|(_, back)| back).sum())
    }

    fn part_b(data: Self::Common) -> anyhow::Result<Self::Answer> {
        Ok(data.into_iter().map(|(front, _)| front).sum())
    }
}

fn extrapolate(history: Vec<i32>) -> anyhow::Result<(i32, i32)> {
    let diff_sequence = iterate(history, |line| {
        line.iter()
            .copied()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect()
    })
    .take_while(|line| line.iter().all_equal_value() != Ok(&0))
    .collect_vec();
    diff_sequence
        .iter()
        .rev()
        .try_fold((0, 0), |(first_bellow, last_bellow), line| {
            Some((line.first()? - first_bellow, line.last()? + last_bellow))
        })
        .ok_or_else(|| anyhow!("Extrapolation failed. Diff sequence: {diff_sequence:?}"))
}

fn history(data: &mut &'static str) -> winnow::PResult<Vec<i32>> {
    separated(1.., dec_int::<_, i32, _>, ' ').parse_next(data)
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
