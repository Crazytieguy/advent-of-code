use std::error::Error;

use itertools::{iterate, Itertools};
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

fn solve(data: &Parsed, decryption_key: i64, iterations: usize) -> i64 {
    let numbers = data
        .iter()
        .map(|&x| x as i64 * decryption_key)
        .collect_vec();
    let mut prev = (0..numbers.len() as u16).collect_vec();
    let mut next = prev.clone();
    prev.rotate_right(1);
    next.rotate_left(1);
    for _ in 0..iterations {
        for (cur, &n) in numbers.iter().enumerate() {
            let amount_to_move = n.rem_euclid(numbers.len() as i64 - 1) as usize;

            // remove cur from the list
            let (cur_prev, cur_next) = (prev[cur], next[cur]);
            prev[cur_next as usize] = cur_prev;
            next[cur_prev as usize] = cur_next;

            // find the node to insert cur after
            let target = iterate(cur_prev, |&cur| next[cur as usize])
                .nth(amount_to_move)
                .unwrap();

            // insert cur after target
            let target_next = next[target as usize];
            prev[cur] = target;
            next[target as usize] = cur as u16;
            prev[target_next as usize] = cur as u16;
            next[cur] = target_next;
        }
    }
    let zero_index = numbers
        .iter()
        .position(|&x| x == 0)
        .expect("an element with value 0") as u16;
    iterate(zero_index, |&cur| next[cur as usize])
        .step_by(1000)
        .skip(1)
        .take(3)
        .map(|i| numbers[i as usize])
        .sum()
}

fn part_a(data: &Parsed) -> i64 {
    solve(data, 1, 1)
}

fn part_b(data: &Parsed) -> i64 {
    solve(data, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
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
