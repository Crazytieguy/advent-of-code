use advent_2023::*;
use anyhow::anyhow;
use itertools::Itertools;

struct Day;

impl BasicSolution for Day {
    const DATA: &'static str = include_str!("data.txt");
    const SAMPLE_DATA: &'static str = include_str!("sample_a.txt");
    const SAMPLE_DATA_B: &'static str = include_str!("sample_b.txt");

    type Parsed = &'static str;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 142;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 281;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        Ok(("", data))
    }

    fn a(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        solve(data, DIGIT_LITERAL_VALUES)
    }

    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        solve(
            data,
            DIGIT_LITERAL_VALUES
                .into_iter()
                .chain(SPELLED_OUT_DIGIT_VALUES),
        )
    }
}

const DIGIT_LITERAL_VALUES: [(&str, u32); 9] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

const SPELLED_OUT_DIGIT_VALUES: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn solve<'a, P>(data: &'a str, patterns: P) -> anyhow::Result<u32>
where
    P: IntoIterator<Item = (&'a str, u32)> + Clone,
    P::IntoIter: Clone,
{
    itertools::process_results(
        data.lines().map(|line| calibration_value(line, &patterns)),
        |it| it.sum(),
    )
}

fn calibration_value<'a, P>(line: &'a str, patterns: &P) -> anyhow::Result<u32>
where
    P: IntoIterator<Item = (&'a str, u32)> + Clone,
    P::IntoIter: Clone,
{
    let err = || anyhow!("Couldn't find a digit in line '{line}'");
    let value_if_pat_at_i = |(i, (pat, value))| line[i..].starts_with(pat).then_some(value);

    let first = (0..line.len())
        .cartesian_product(patterns.clone())
        .find_map(value_if_pat_at_i)
        .ok_or_else(err)?;

    let last = (0..line.len())
        .rev()
        .cartesian_product(patterns.clone())
        .find_map(value_if_pat_at_i)
        .ok_or_else(err)?;

    Ok(first * 10 + last)
}

fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_b()
    }
}
