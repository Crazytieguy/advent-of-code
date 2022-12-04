use nom::{
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::{error::Error, ops::RangeInclusive};

const DATA: &str = include_str!("data.txt");

type Parsed = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

fn parse(data: &'static str) -> IResult<&'static str, Parsed> {
    let pair_to_range = |(a, b)| a..=b;
    separated_list1(
        line_ending,
        separated_pair(
            separated_pair(complete::u32, complete::char('-'), complete::u32).map(pair_to_range),
            complete::char(','),
            separated_pair(complete::u32, complete::char('-'), complete::u32).map(pair_to_range),
        ),
    )(data)
}

fn one_range_contains_the_other((a, b): &&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) & b.contains(a.end())
}

fn ranges_overlap((a, b): &&(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}

fn part_a(data: &Parsed) -> usize {
    data.iter().filter(one_range_contains_the_other).count()
}

fn part_b(data: &Parsed) -> usize {
    data.iter().filter(ranges_overlap).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 2);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 4);
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
