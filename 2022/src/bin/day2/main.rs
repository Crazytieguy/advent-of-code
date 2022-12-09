use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom_supreme::ParserExt;
use std::error::Error;

const DATA: &str = include_str!("data.txt");

#[derive(Debug, Clone, Copy)]
enum Left {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy)]
enum Right {
    X,
    Y,
    Z,
}

type Parsed = Vec<(Left, Right)>;

fn parse(data: &'static str) -> IResult<&'static str, Parsed> {
    separated_list1(
        line_ending,
        separated_pair(
            alt((
                char('A').value(Left::A),
                char('B').value(Left::B),
                char('C').value(Left::C),
            )),
            char(' '),
            alt((
                char('X').value(Right::X),
                char('Y').value(Right::Y),
                char('Z').value(Right::Z),
            )),
        ),
    )(data)
}

fn part_a(data: &Parsed) -> usize {
    data.iter()
        .map(|&(opponent_choice, own_choice)| {
            let result_score = match (own_choice as i8 - opponent_choice as i8).rem_euclid(3) {
                0 => 3, // tie
                1 => 6, // win
                2 => 0, // lose
                _ => unreachable!(),
            };
            let choice_score = own_choice as usize + 1;
            choice_score + result_score
        })
        .sum()
}

fn part_b(data: &Parsed) -> usize {
    data.iter()
        .map(|&(opponent_choice, result)| {
            let choice_score =
                (opponent_choice as i8 + result as i8 - 1).rem_euclid(3) as usize + 1;
            let result_score = result as usize * 3;
            choice_score + result_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 15);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 12);
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
