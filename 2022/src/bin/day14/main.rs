use std::{collections::HashSet, error::Error};

use nom::{
    bytes::complete::tag,
    character::complete::{char, u16, u8},
    multi::separated_list1,
    sequence::separated_pair,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = HashSet<(u16, u8)>;

fn parse_coords(data: &str) -> IResult<(u16, u8)> {
    separated_pair(u16, char(','), u8)(data)
}

fn parse_continuous_segments(data: &str) -> IResult<Vec<(u16, u8)>> {
    separated_list1(tag(" -> "), parse_coords)(data)
}

fn parse(data: &str) -> IResult<Parsed> {
    let mut rocks = HashSet::new();
    for line in data.lines() {
        parse_continuous_segments(line)?
            .1
            .into_iter()
            .reduce(|(x1, y1), (x2, y2)| {
                let (xmin, xmax) = (x1.min(x2), x1.max(x2));
                let (ymin, ymax) = (y1.min(y2), y1.max(y2));
                for x in xmin..=xmax {
                    for y in ymin..=ymax {
                        rocks.insert((x, y));
                    }
                }
                (x2, y2)
            });
    }
    Ok(("", rocks))
}

fn part_a(data: &Parsed) -> usize {
    let mut full_coords = data.clone();
    let num_rocks = data.len();
    let &max_y = data
        .iter()
        .map(|(_, y)| y)
        .max()
        .expect("At least one rock");
    'outer: loop {
        let mut x = 500;
        for y in 0..max_y {
            if let Some(new_x) = [x, x - 1, x + 1]
                .iter()
                .find(|&&x| !full_coords.contains(&(x, y + 1)))
            {
                x = *new_x;
            } else {
                full_coords.insert((x, y));
                continue 'outer;
            }
        }
        break;
    }
    full_coords.len() - num_rocks
}

fn part_b(data: &Parsed) -> usize {
    let mut full_coords = data.clone();
    let num_rocks = data.len();
    let &max_y = data
        .iter()
        .map(|(_, y)| y)
        .max()
        .expect("At least one rock");
    'outer: while !full_coords.contains(&(500, 0)) {
        let mut x = 500;
        for y in 0..max_y + 2 {
            if let Some(new_x) = [x, x - 1, x + 1]
                .iter()
                .find(|&&x| !full_coords.contains(&(x, y + 1)))
            {
                x = *new_x;
            } else {
                full_coords.insert((x, y));
                continue 'outer;
            }
        }
        full_coords.insert((x, max_y + 1));
    }
    full_coords.len() - num_rocks
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
