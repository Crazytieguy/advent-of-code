use advent_2023::*;
use anyhow::anyhow;

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
        solve(data, &[])
    }

    fn b(data: Self::Parsed) -> anyhow::Result<Self::Answer> {
        solve(
            data,
            &[
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ],
        )
    }
}

fn solve(data: &str, spelled_out_vals: &[(&str, u32)]) -> anyhow::Result<u32> {
    itertools::process_results(
        data.lines()
            .map(|line| calibration_value(line, spelled_out_vals)),
        |it| it.sum(),
    )
}

fn calibration_value(line: &str, spelled_out_vals: &[(&str, u32)]) -> anyhow::Result<u32> {
    let err = || anyhow!("Couldn't find a digit in line '{line}'");

    let digit_at_i = |i| {
        let literal = line[i..i + 1].parse().ok();
        let match_spelled = |&(digit, val)| line[i..].starts_with(digit).then_some(val);
        literal.or_else(|| spelled_out_vals.iter().find_map(match_spelled))
    };

    let first = (0..line.len()).find_map(digit_at_i).ok_or_else(err)?;
    let last = (0..line.len()).rev().find_map(digit_at_i).ok_or_else(err)?;

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
