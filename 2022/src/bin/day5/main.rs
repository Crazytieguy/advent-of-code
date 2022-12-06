use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{char, line_ending, one_of, u8},
        streaming::not_line_ending,
    },
    multi::{many1_count, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    Parser,
};
use nom_supreme::{multi::parse_separated_terminated, ParserExt};
use std::{collections::VecDeque, error::Error};

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

const DATA: &str = include_str!("data.txt");

struct Instruction {
    amount: u8,
    from: u8,
    to: u8,
}

type Stacks = Vec<VecDeque<char>>;
type Parsed = (Stacks, Vec<Instruction>);

fn crate_(input: &str) -> IResult<Option<char>> {
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let crate_ = delimited(char('['), one_of(UPPERCASE), char(']')).map(Some);
    let not_crate = tag("   ").value(None);
    alt((crate_, not_crate))(input)
}

fn stacks(input: &str) -> IResult<Stacks> {
    let mut stacks = vec![];
    let crate_row = parse_separated_terminated(
        crate_,
        char(' '),
        line_ending,
        || 0,
        |i, opt_crate| {
            if i == stacks.len() {
                stacks.push(VecDeque::new());
            }
            if let Some(c) = opt_crate {
                stacks[i].push_front(c);
            }
            i + 1
        },
    );
    let (input, _) = many1_count(crate_row)(input)?;
    let (input, _) = not_line_ending.terminated(line_ending).parse(input)?;
    Ok((input, stacks))
}

fn instruction(input: &str) -> IResult<Instruction> {
    let (input, (_, amount, _, from, _, to)) =
        tuple((tag("move "), u8, tag(" from "), u8, tag(" to "), u8))(input)?;
    let (from, to) = (from - 1, to - 1);
    Ok((input, Instruction { amount, from, to }))
}

fn instructions(input: &str) -> IResult<Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_pair(stacks, line_ending, instructions)(data)
}

fn get_top_crates(stacks: Stacks) -> String {
    stacks
        .into_iter()
        .map(|stack| {
            *stack
                .back()
                .expect("each stack should have at least one crate")
        })
        .collect()
}

fn solve(stacks: &Stacks, instructions: &[Instruction], reverse_order: bool) -> String {
    let mut stacks = stacks.clone();
    let mut temp_stack = VecDeque::new();
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
