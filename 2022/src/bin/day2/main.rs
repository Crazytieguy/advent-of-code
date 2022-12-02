use nom::{
    bytes::complete::take,
    character::complete::{char, line_ending},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use serde::Deserialize;

const DATA: &str = include_str!("data.txt");

#[derive(Debug, Clone, Copy, Deserialize)]
enum Left {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, Deserialize)]
enum Right {
    X,
    Y,
    Z,
}

type Parsed = Vec<(Left, Right)>;

fn parse(data: &'static str) -> IResult<&'static str, Parsed> {
    separated_list0(
        line_ending,
        separated_pair(
            map_res(take(1usize), serde_plain::from_str),
            char(' '),
            map_res(take(1usize), serde_plain::from_str),
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

fn parse_unwrap(data: &'static str) -> Parsed {
    parse(data).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(&parse_unwrap(SAMPLE_DATA)), 15);
        println!("part a: {}", part_a(&parse_unwrap(DATA)));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&parse_unwrap(SAMPLE_DATA)), 12);
        println!("part b: {}", part_b(&parse_unwrap(DATA)));
    }
}

fn main() {
    let parsed = parse_unwrap(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
}
