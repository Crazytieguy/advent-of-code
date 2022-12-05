use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, line_ending, multispace0, one_of, u8},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
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

fn parse_stacks(input: &str) -> IResult<Stacks> {
    let (input, initial_state) = take_until("\n\n")(input)?;
    let mut stacks = vec![];
    for mut line in initial_state.lines().dropping_back(1) {
        for i in 0.. {
            let (rest, c) = parse_crate(line)?;
            if stacks.len() == i {
                stacks.push(vec![]);
            }
            if let Some(c) = c {
                stacks[i].push(c);
            }
            if rest.is_empty() {
                break;
            }
            (line, _) = char(' ')(rest)?;
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
        multispace0,
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

fn solve(stacks: &Stacks, instructions: &[Instruction], reverse_order: bool) -> String {
    let mut stacks = stacks.clone();
    let mut temp_stack = vec![];
    for &Instruction { amount, from, to } in instructions {
        let (amount, from, to) = (amount as usize, from as usize, to as usize);
        let at = stacks[from].len() - amount;
        temp_stack.extend(stacks[from].drain(at..));
        if reverse_order {
            stacks[to].extend(temp_stack.drain(..).rev());
        } else {
            stacks[to].append(&mut temp_stack);
        }
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
