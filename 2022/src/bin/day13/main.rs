use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, line_ending, u8},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, separated_pair},
    Parser,
};
use std::{cmp::Ordering, error::Error};
use Value::*;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = Vec<(Value, Value)>;

#[derive(Debug, Clone)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (List(a), List(b)) => a.cmp(b),
            (Integer(a), list) => List(vec![Integer(*a)]).cmp(list),
            (list, Integer(b)) => list.cmp(&List(vec![Integer(*b)])),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_value(data: &str) -> IResult<Value> {
    alt((
        u8.map(Integer),
        delimited(
            char('['),
            separated_list0(char(','), parse_value),
            char(']'),
        )
        .map(List),
    ))(data)
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(
        pair(line_ending, line_ending),
        separated_pair(parse_value, line_ending, parse_value),
    )(data)
}

fn part_a(data: &Parsed) -> usize {
    data.iter()
        .enumerate()
        .filter(|(_, (a, b))| b >= a)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_b(data: &Parsed) -> usize {
    let divider_a = List(vec![List(vec![Integer(2)])]);
    let divider_b = List(vec![List(vec![Integer(6)])]);
    let all_packets = data
        .iter()
        .cloned()
        .flat_map(|(a, b)| [a, b])
        .chain([divider_a.clone(), divider_b.clone()])
        .sorted_unstable()
        .collect_vec();
    all_packets.partition_point(|v| v <= &divider_a)
        * all_packets.partition_point(|v| v <= &divider_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 13);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 140);
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
