use advent_2022::*;
use itertools::{iterate, Itertools};
use nom::{
    character::complete::{i64, line_ending},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl Solution for Day {
    type Parsed = Vec<i64>;
    type A = i64;
    type B = i64;
    const SAMPLE_ANSWER_A: Self::TestA = 3;
    const SAMPLE_ANSWER_B: Self::TestB = 1623178306;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, i64)
            .terminated(line_ending)
            .parse(data)
    }

    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest> {
        Self::parse(data)
    }

    fn a(data: Self::Parsed) -> Self::A {
        solve::<25>(data, 1, 1)
    }

    fn b(data: Self::Parsed) -> Self::B {
        solve::<25>(data, 811589153, 10)
    }

    fn a_test(data: Self::ParsedTest) -> Self::TestA {
        solve::<1>(data, 1, 1)
    }

    fn b_test(data: Self::ParsedTest) -> Self::TestB {
        solve::<1>(data, 811589153, 10)
    }
}

fn solve<const NEXT_SIZE: usize>(data: Vec<i64>, decryption_key: i64, iterations: usize) -> i64 {
    let numbers = data.into_iter().map(|x| x * decryption_key).collect_vec();
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
