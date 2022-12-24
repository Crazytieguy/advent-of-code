use std::error::Error;

use itertools::Itertools;
use nom::{
    character::complete::{i32, line_ending},
    multi::separated_list1,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = Vec<i32>;

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, i32)(data)
}

fn part_a(data: &Parsed) -> i32 {
    let mut reorder = data.iter().map(|&x| (x, false)).collect_vec();
    let mut index = 0;
    while index < data.len() {
        if reorder[index].1 {
            index += 1;
            continue;
        }
        let mut num = reorder.remove(index);
        num.1 = true;
        let new_index = (index as i32 + num.0).rem_euclid(reorder.len() as i32);
        reorder.insert(new_index as usize, num);
    }
    let zero_index = reorder
        .iter()
        .position(|&(x, _)| x == 0)
        .expect("an element with value 0");
    dbg!(zero_index);
    (zero_index..)
        .step_by(1000)
        .skip(1)
        .take(3)
        .map(|i| reorder[i % reorder.len()].0)
        .inspect(|n| {
            dbg!(*n);
        })
        .sum()
}

fn part_b(data: &Parsed) -> i64 {
    let numbers = data.iter().map(|&x| x as i64 * 811589153).collect_vec();
    let mut positions = (0..numbers.len()).collect_vec();
    for _ in 0..10 {
        for (i, &n) in numbers.iter().enumerate() {
            let cur_position = positions[i];
            let new_position =
                (cur_position as i64 + n).rem_euclid(numbers.len() as i64 - 1) as usize;
            positions.iter_mut().for_each(|p| {
                if *p > cur_position && *p <= new_position {
                    *p -= 1
                }
                if *p < cur_position && *p >= new_position {
                    *p += 1
                }
            });
            positions[i] = new_position;
        }
    }
    let mut ordered_numbers = vec![0; numbers.len()];
    for (i, &p) in positions.iter().enumerate() {
        ordered_numbers[p] = numbers[i];
    }
    let zero_index = ordered_numbers
        .iter()
        .position(|&x| x == 0)
        .expect("an element with value 0");
    (zero_index..)
        .step_by(1000)
        .skip(1)
        .take(3)
        .map(|i| ordered_numbers[i % ordered_numbers.len()])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    #[ignore]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 3);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 1623178306);
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
