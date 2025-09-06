use advent_2022::*;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<(i8, i8)>;
    type Answer = u32;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 15;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 12;

    fn parse(data: &str) -> IResult<'_, Self::Parsed> {
        separated_list1(
            line_ending,
            separated_pair(
                alt((char('A').value(0), char('B').value(1), char('C').value(2))),
                char(' '),
                alt((char('X').value(0), char('Y').value(1), char('Z').value(2))),
            ),
        )(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        data.into_iter()
            .map(|(opponent_choice, own_choice)| {
                let result_score = match (own_choice - opponent_choice).rem_euclid(3) {
                    0 => 3, // tie
                    1 => 6, // win
                    2 => 0, // lose
                    _ => unreachable!(),
                };
                let choice_score = own_choice as u32 + 1;
                choice_score + result_score
            })
            .sum()
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        data.into_iter()
            .map(|(opponent_choice, result)| {
                let choice_score = (opponent_choice + result - 1).rem_euclid(3) as u32 + 1;
                let result_score = result as u32 * 3;
                choice_score + result_score
            })
            .sum()
    }
}
