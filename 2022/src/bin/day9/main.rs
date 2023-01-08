use advent_2022::*;
use itertools::{repeat_n, Itertools};
use nom::{
    branch::alt,
    character::complete::{char, line_ending, u8},
    multi::separated_list1,
    sequence::separated_pair,
    Parser,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<(Direction, u8)>;
    type A = usize;
    type B = usize;
    const SAMPLE_ANSWER_A: Self::TestA = 13;
    const SAMPLE_ANSWER_B: Self::TestB = 1;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, movement)
            .terminated(line_ending)
            .parse(data)
    }

    fn a(data: Self::Parsed) -> Self::A {
        solve::<2>(&data)
    }

    fn b(data: Self::Parsed) -> Self::B {
        solve::<10>(&data)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}
use Direction::*;

fn movement(input: &str) -> IResult<(Direction, u8)> {
    separated_pair(
        alt((
            char('R').value(Right),
            char('U').value(Up),
            char('L').value(Left),
            char('D').value(Down),
        )),
        char(' '),
        u8,
    )(input)
}

type Point = (i16, i16);

fn solve<const N: usize>(data: &[(Direction, u8)]) -> usize {
    let mut points = [(0, 0); N];
    data.iter()
        .flat_map(|&(direction, amount)| repeat_n(direction, amount as usize))
        .map(|direction| {
            reposition_head(direction, &mut points[0]);
            (0..N - 1).for_each(|i| follow_knot(points[i], &mut points[i + 1]));
            points[N - 1]
        })
        .unique()
        .count()
}

fn follow_knot(leader: Point, follower: &mut Point) {
    let diff = (leader.0 - follower.0, leader.1 - follower.1);
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        follower.0 += diff.0.signum();
        follower.1 += diff.1.signum();
    }
}

fn reposition_head(direction: Direction, head: &mut Point) {
    match direction {
        Right => head.0 += 1,
        Up => head.1 += 1,
        Left => head.0 -= 1,
        Down => head.1 -= 1,
    }
}

#[cfg(test)]
#[test]
fn extra_test_b() -> OutResult {
    const DATA: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    let parsed = Day::final_parse(DATA)?;
    assert_eq!(<Day as Solution>::b(parsed), 36);
    Ok(())
}
