#![feature(array_windows)]
use std::borrow::Cow;

use advent_2024::{BasicSolution, Solution};
use itertools::izip;

struct Day;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<&'static [u8]>;
    type Answer = usize;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 18;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 9;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        Ok(input.lines().map(str::as_bytes).collect())
    }

    fn part_a(shared: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        const LOOK_FOR: [&[u8; 4]; 2] = [b"XMAS", b"SAMX"];
        Ok(shared
            .array_windows()
            .enumerate()
            .flat_map(|(i, &[a, b, c, d])| {
                let vertical = izip!(a, b, c, d);
                let diag1 = izip!(a, &b[1..], &c[2..], &d[3..]);
                let diag2 = izip!(&a[3..], &b[2..], &c[1..], d);

                let horizontals = if i == 0 {
                    [a, b, c, d]
                } else {
                    [&[], &[], &[], d]
                }
                .into_iter()
                .flat_map(|line| line.array_windows())
                .copied();

                vertical
                    .chain(diag1)
                    .chain(diag2)
                    .map(|(&a, &b, &c, &d)| [a, b, c, d])
                    .chain(horizontals)
            })
            .filter(|word| LOOK_FOR.contains(&word))
            .count())
    }

    fn part_b(shared: Self::Shared) -> anyhow::Result<Self::Answer> {
        const LOOK_FOR: [&[u8; 3]; 2] = [b"MAS", b"SAM"];
        Ok(shared
            .array_windows()
            .flat_map(|&[a, b, c]| izip!(a, &a[2..], &b[1..], c, &c[2..]))
            .filter(|&(&tl, &tr, &m, &bl, &br)| {
                LOOK_FOR.contains(&&[tl, m, br]) && LOOK_FOR.contains(&&[tr, m, bl])
            })
            .count())
    }
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
