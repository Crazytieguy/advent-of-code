use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, one_of, space0, u8},
    multi::{many1_count, separated_list1},
    sequence::{delimited, pair, separated_pair, terminated, tuple},
    IResult, Parser,
};
use std::error::Error;

const DATA: &str = include_str!("data.txt");

struct Instruction {
    amount: u8,
    from: usize,
    to: usize,
}
type Parsed = (Vec<Vec<char>>, Vec<Instruction>);

fn parse_crate(input: &str) -> IResult<&str, Option<char>> {
    alt((
        delimited(char('['), one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ"), char(']')).map(Some),
        tag("   ").map(|_| None),
    ))(input)
}

fn parse_crates_row(input: &str) -> IResult<&str, Vec<Option<char>>> {
    separated_list1(char(' '), parse_crate)(input)
}

fn parse_stack_numbers(input: &str) -> IResult<&str, usize> {
    many1_count(delimited(space0, u8, space0))(input)
}

fn parse_initial_state(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, (stacks, num_stacks)) = terminated(
        separated_pair(
            separated_list1(line_ending, parse_crates_row),
            line_ending,
            parse_stack_numbers,
        ),
        pair(line_ending, line_ending),
    )(input)?;
    let mut result = vec![vec![]; num_stacks];
    for row in stacks {
        for (i, crate_) in row.into_iter().enumerate() {
            if let Some(c) = crate_ {
                result[i].push(c);
            }
        }
    }
    result.iter_mut().for_each(|s| s.reverse());
    Ok((input, result))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, amount, _, from, _, to)) =
        tuple((tag("move "), u8, tag(" from "), u8, tag(" to "), u8))(input)?;
    Ok((
        input,
        Instruction {
            amount,
            from: from as usize - 1,
            to: to as usize - 1,
        },
    ))
}

fn parse(data: &'static str) -> IResult<&'static str, Parsed> {
    pair(
        parse_initial_state,
        separated_list1(line_ending, parse_instruction),
    )(data)
}

fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .map(|stack| {
            *stack
                .last()
                .expect("each stack should have at least one crate")
        })
        .collect()
}

fn move_crates(stacks: &mut [Vec<char>], instructions: &[Instruction], reverse: bool) {
    let mut temp_stack = vec![];
    for instruction in instructions {
        let range = stacks[instruction.from].len() - instruction.amount as usize..;
        let iter_crates = stacks[instruction.from].drain(range);
        if reverse {
            temp_stack.extend(iter_crates.rev());
        } else {
            temp_stack.extend(iter_crates);
        }
        stacks[instruction.to].append(&mut temp_stack);
    }
}

fn part_a((stacks, instructions): &Parsed) -> String {
    let mut stacks = stacks.clone();
    move_crates(&mut stacks, instructions, true);
    get_top_crates(stacks)
}

fn part_b((stacks, instructions): &Parsed) -> String {
    let mut stacks = stacks.clone();
    move_crates(&mut stacks, instructions, false);
    get_top_crates(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), "CMZ");
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), "MCD");
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
