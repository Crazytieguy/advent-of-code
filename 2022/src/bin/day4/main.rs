use nom::{
    character::complete::{char, line_ending, u32},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::ParserExt;
use std::{error::Error, ops::RangeInclusive};

const DATA: &str = include_str!("data.txt");

type RangesPair = [RangeInclusive<u32>; 2];
type Parsed = Vec<RangesPair>;

fn parse(data: &'static str) -> IResult<&'static str, Parsed> {
    separated_list1(
        line_ending,
        u32.separated_array(char('-'))
            .map(|[a, b]| a..=b)
            .separated_array(char(',')),
    )(data)
}

fn one_range_contains_the_other([a, b]: &&RangesPair) -> bool {
    a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) && b.contains(a.end())
}

fn ranges_overlap([a, b]: &&RangesPair) -> bool {
    a.contains(b.start()) || a.contains(b.end())
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
