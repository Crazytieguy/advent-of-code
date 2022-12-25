use std::error::Error;

use itertools::{iterate, Itertools};
use nom::{
    character::complete::{i64, line_ending},
    multi::separated_list1,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = Vec<i64>;

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, i64)(data)
}

fn solve<const NEXT_SIZE: usize>(data: &Parsed, decryption_key: i64, iterations: usize) -> i64 {
    let numbers = data.iter().map(|&x| x * decryption_key).collect_vec();
    let mut prev = (0..numbers.len() as u16).collect_vec();
    let mut next = prev.clone();
    prev.rotate_right(1);
    next.rotate_left(NEXT_SIZE % numbers.len());
    for _ in 0..iterations {
        for (cur, &n) in numbers.iter().enumerate() {
            // remove cur from the list
            fix_pairs_backwards(prev[cur], next[cur], &mut prev, &mut next, cur as u16);

            // find the node to insert cur after
            let amount_to_move = n.rem_euclid(numbers.len() as i64 - 1) as usize;
            let target = find_target::<NEXT_SIZE>(prev[cur], amount_to_move, &prev, &next);

            // insert cur after target
            prev[cur] = target;
            fix_pairs_backwards(
                cur as u16,
                next[target as usize],
                &mut prev,
                &mut next,
                target,
            );
        }
    }
    let zero_index = numbers
        .iter()
        .position(|&x| x == 0)
        .expect("an element with value 0");
    iterate(zero_index as u16, |&cur| {
        find_target::<NEXT_SIZE>(cur, 1000, &prev, &next)
    })
    .skip(1)
    .take(3)
    .map(|i| numbers[i as usize])
    .sum()
}

fn fix_pairs_backwards(left: u16, right: u16, prev: &mut [u16], next: &mut [u16], stop: u16) {
    let (far_prev, immediate_next) = iterate(left, |&i| prev[i as usize])
        .zip(iterate(right, |&i| prev[i as usize]))
        .inspect(|&(before, after)| {
            next[before as usize] = after;
        })
        .find(|&(_, after)| prev[after as usize] == stop)
        .unwrap();
    prev[immediate_next as usize] = left;
    next[prev[far_prev as usize] as usize] = left;
}

fn find_target<const NEXT_SIZE: usize>(
    from: u16,
    amount_to_move: usize,
    prev: &[u16],
    next: &[u16],
) -> u16 {
    let overshot_target = iterate(from, |&cur| next[cur as usize])
        .nth((NEXT_SIZE + amount_to_move) / NEXT_SIZE)
        .unwrap();
    iterate(overshot_target, |&cur| prev[cur as usize])
        .nth(NEXT_SIZE - amount_to_move % NEXT_SIZE)
        .unwrap()
}

fn part_a<const NEXT_SIZE: usize>(data: &Parsed) -> i64 {
    solve::<NEXT_SIZE>(data, 1, 1)
}

fn part_b<const NEXT_SIZE: usize>(data: &Parsed) -> i64 {
    solve::<NEXT_SIZE>(data, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a::<1>(&parse(SAMPLE_DATA)?.1), 3);
        println!("part a: {}", part_a::<25>(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b::<1>(&parse(SAMPLE_DATA)?.1), 1623178306);
        println!("part b: {}", part_b::<25>(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a::<25>(&parsed));
    println!("part b: {}", part_b::<25>(&parsed));
    Ok(())
}
