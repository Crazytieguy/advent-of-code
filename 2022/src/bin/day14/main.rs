use std::{collections::HashSet, error::Error};

use itertools::process_results;
use nom::{
    character::complete::{char, u16, u8},
    sequence::separated_pair,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = HashSet<(u16, u8)>;

fn parse_coords(data: &str) -> IResult<(u16, u8)> {
    separated_pair(u16, char(','), u8)(data)
}

fn parse(data: &str) -> IResult<Parsed> {
    let mut rocks = HashSet::new();
    for line in data.lines() {
        process_results(line.split(" -> ").map(parse_coords), |it| {
            it.map(|(_, xy)| xy).reduce(|(x1, y1), (x2, y2)| {
                for x in x1.min(x2)..=x1.max(x2) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        rocks.insert((x, y));
                    }
                }
                (x2, y2)
            })
        })?;
    }
    Ok(("", rocks))
}

fn find_resting_spot(taken_coords: &Parsed, max_y: u8) -> (u16, u8) {
    let mut x = 500;
    for y in 1..=max_y {
        if let Some(new_x) = [x, x - 1, x + 1]
            .iter()
            .find(|&&x| !taken_coords.contains(&(x, y)))
        {
            x = *new_x;
        } else {
            return (x, y - 1);
        }
    }
    (x, max_y)
}

fn solve(data: &Parsed, stop_condition: impl Fn(u8, u8) -> bool) -> usize {
    let num_rocks = data.len();
    let max_y = data
        .iter()
        .map(|(_, y)| y)
        .max()
        .expect("At least one rock")
        + 1;
    let mut taken_coords = data.clone();
    loop {
        let (x, y) = find_resting_spot(&taken_coords, max_y);
        if stop_condition(y, max_y) {
            break;
        }
        taken_coords.insert((x, y));
    }
    taken_coords.len() - num_rocks
}

fn part_a(data: &Parsed) -> usize {
    solve(data, |y, max_y| y == max_y)
}

fn part_b(data: &Parsed) -> usize {
    solve(data, |y, _| y == 0) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 24);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 93);
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
