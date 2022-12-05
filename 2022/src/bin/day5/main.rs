use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, one_of, space0, u8},
    multi::{many1_count, separated_list1},
    sequence::{delimited, pair, separated_pair, tuple},
    Parser,
};
use std::error::Error;

type OutResult = std::result::Result<(), Box<dyn Error>>;

const DATA: &str = include_str!("data.txt");

struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

type Stacks = Vec<Vec<char>>;

type Parsed = (Stacks, Vec<Instruction>);

type IResult<'a, T> = nom::IResult<&'a str, T>;

fn parse_crate(input: &str) -> IResult<Option<char>> {
    alt((
        delimited(char('['), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), char(']')).map(Some),
        tag("   ").map(|_| None),
    ))(input)
}

fn parse_crates_row(input: &str) -> IResult<Vec<Option<char>>> {
    separated_list1(char(' '), parse_crate)(input)
}

fn parse_stack_numbers(input: &str) -> IResult<usize> {
    many1_count(delimited(space0, u8, space0))(input)
}

fn parse_stacks(input: &str) -> IResult<Stacks> {
    let (input, (rows, num_stacks)) = separated_pair(
        separated_list1(line_ending, parse_crates_row),
        line_ending,
        parse_stack_numbers,
    )(input)?;
    let mut stacks = vec![vec![]; num_stacks];
    for row in rows {
        for (slot, stack) in row.into_iter().zip(stacks.iter_mut()) {
            if let Some(crate_) = slot {
                stack.push(crate_)
            }
        }
    }
    stacks.iter_mut().for_each(|s| s.reverse());
    Ok((input, stacks))
}

fn parse_instruction(input: &str) -> IResult<Instruction> {
    let (input, (_, amount, _, from, _, to)) =
        tuple((tag("move "), u8, tag(" from "), u8, tag(" to "), u8))(input)?;
    let (from, to) = (from - 1, to - 1);
    Ok((input, Instruction { amount, from, to }))
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_pair(
        parse_stacks,
        pair(line_ending, line_ending),
        separated_list1(line_ending, parse_instruction),
    )(data)
}

fn get_top_crates(stacks: Stacks) -> String {
    stacks
        .into_iter()
        .map(|stack| {
            *stack
                .last()
                .expect("each stack should have at least one crate")
        })
        .collect()
}

fn solve(stacks: &Stacks, instructions: &[Instruction], reverse_when_moving: bool) -> String {
    let mut stacks = stacks.clone();
    let mut temp_stack = vec![];
    for &Instruction { amount, from, to } in instructions {
        let (amount, from, to) = (amount as usize, from as usize, to as usize);
        let range = stacks[from].len() - amount..;
        let crates_to_move = stacks[from].drain(range);
        if reverse_when_moving {
            temp_stack.extend(crates_to_move.rev());
        } else {
            temp_stack.extend(crates_to_move);
        }
        stacks[to].append(&mut temp_stack);
    }
    get_top_crates(stacks)
}

fn part_a((stacks, instructions): &Parsed) -> String {
    solve(stacks, instructions, true)
}

fn part_b((stacks, instructions): &Parsed) -> String {
    solve(stacks, instructions, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), "CMZ");
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), "MCD");
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
