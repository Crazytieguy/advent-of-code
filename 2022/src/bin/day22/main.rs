use advent_2022::*;
use itertools::iterate;
use nom::{
    branch::alt,
    character::complete::{char, u32},
    multi::many1,
    Parser,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl Solution for Day {
    type Parsed = (Vec<&'static [u8]>, Vec<Move>);
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 6032;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 5031;

    fn parse(data: &'static str) -> IResult<'static, Self::Parsed> {
        let mut lines = data.lines();
        let grid = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(str::as_bytes)
            .collect();
        let path = lines
            .next()
            .expect("there should be a path after the empty line");
        many1(parse_move)
            .parse(path)
            .map(|(rest, moves)| (rest, (grid, moves)))
    }

    fn parse_test(data: &'static str) -> IResult<'static, Self::ParsedTest> {
        Self::parse(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        solve(data, move_one_2d::<150, 200>)
    }

    fn a_test(data: Self::ParsedTest) -> Self::TestAnswer {
        solve(data, move_one_2d::<16, 12>)
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        solve(data, move_one_cube)
    }

    fn b_test(data: Self::ParsedTest) -> Self::TestAnswer {
        solve(data, move_one_sample_cube)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}
use Rotation::*;

#[derive(Debug, Clone, Copy)]
enum Move {
    Turn(Rotation),
    Forward(usize),
}
use Move::*;

fn parse_move(input: &str) -> IResult<Move> {
    alt((
        char('R').value(Turn(Clockwise)),
        char('L').value(Turn(CounterClockwise)),
        u32.map(|n| Forward(n as usize)),
    ))(input)
}

#[derive(Debug, Clone, Copy)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
}

fn solve((grid, path): (Vec<&'static [u8]>, Vec<Move>), move_one: fn(&State) -> State) -> usize {
    let State { x, y, direction } = path.into_iter().fold(
        State {
            y: 0,
            x: grid[0].iter().position(|&c| c == b'.').unwrap(),
            direction: Right,
        },
        |state, m| match m {
            Turn(Clockwise) => State {
                direction: match state.direction {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                },
                ..state
            },
            Turn(CounterClockwise) => State {
                direction: match state.direction {
                    Up => Left,
                    Left => Down,
                    Down => Right,
                    Right => Up,
                },
                ..state
            },
            Forward(n) => iterate(state, move_one)
                .filter(|s| *grid[s.y].get(s.x).unwrap_or(&b' ') != b' ')
                .take(n + 1)
                .take_while(|s| grid[s.y][s.x] == b'.')
                .last()
                .unwrap(),
        },
    );
    let row_number = y + 1;
    let column_number = x + 1;
    let facing_number = match direction {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    };
    1000 * row_number + 4 * column_number + facing_number
}

fn move_one_2d<const X: usize, const Y: usize>(&State { x, y, direction }: &State) -> State {
    match direction {
        Right => State {
            x: (x + 1) % X,
            y,
            direction,
        },
        Down => State {
            x,
            y: (y + 1) % Y,
            direction,
        },
        Left => State {
            x: x.checked_sub(1).unwrap_or(X - 1),
            y,
            direction,
        },
        Up => State {
            x,
            y: y.checked_sub(1).unwrap_or(Y - 1),
            direction,
        },
    }
}

fn move_one_cube(&State { x, y, direction }: &State) -> State {
    match direction {
        Right => match (x, y) {
            (149, 0..50) => State {
                x: 99,
                y: 149 - y,
                direction: Left,
            },
            (99, 50..100) => State {
                x: 100 + (y - 50),
                y: 49,
                direction: Up,
            },
            (99, 100..150) => State {
                x: 149,
                y: 49 - (y - 100),
                direction: Left,
            },
            (49, 150..200) => State {
                x: 50 + (y - 150),
                y: 149,
                direction: Up,
            },
            _ => State {
                x: x + 1,
                y,
                direction,
            },
        },
        Left => match (x, y) {
            (50, 0..50) => State {
                x: 0,
                y: 149 - y,
                direction: Right,
            },
            (50, 50..100) => State {
                x: y - 50,
                y: 100,
                direction: Down,
            },
            (0, 100..150) => State {
                x: 50,
                y: 49 - (y - 100),
                direction: Right,
            },
            (0, 150..200) => State {
                x: 50 + (y - 150),
                y: 0,
                direction: Down,
            },
            _ => State {
                x: x - 1,
                y,
                direction,
            },
        },
        Down => match (x, y) {
            (0..50, 199) => State {
                x: x + 100,
                y: 0,
                direction: Down,
            },
            (50..100, 149) => State {
                x: 49,
                y: 150 + (x - 50),
                direction: Left,
            },
            (100..150, 49) => State {
                x: 99,
                y: 50 + (x - 100),
                direction: Left,
            },
            _ => State {
                x,
                y: y + 1,
                direction,
            },
        },
        Up => match (x, y) {
            (0..50, 100) => State {
                x: 50,
                y: 50 + x,
                direction: Right,
            },
            (50..100, 0) => State {
                x: 0,
                y: 150 + (x - 50),
                direction: Right,
            },
            (100..150, 0) => State {
                x: x - 100,
                y: 199,
                direction: Up,
            },
            _ => State {
                x,
                y: y - 1,
                direction,
            },
        },
    }
}

fn move_one_sample_cube(&State { x, y, direction }: &State) -> State {
    match direction {
        Right => match (x, y) {
            (11, 0..4) => State {
                x: 15,
                y: 11 - y,
                direction: Left,
            },
            (11, 4..8) => State {
                x: 15 - (y - 4),
                y: 8,
                direction: Down,
            },
            (15, 8..12) => State {
                x: 11,
                y: 3 - (y - 8),
                direction: Left,
            },
            _ => State {
                x: x + 1,
                y,
                direction,
            },
        },
        Left => match (x, y) {
            (8, 0..4) => State {
                x: y + 4,
                y: 4,
                direction: Down,
            },
            (0, 4..8) => State {
                x: 15 - (y - 4),
                y: 11,
                direction: Up,
            },
            (8, 8..12) => State {
                x: 7 - (y - 8),
                y: 7,
                direction: Up,
            },
            _ => State {
                x: x - 1,
                y,
                direction,
            },
        },
        Down => match (x, y) {
            (0..4, 7) => State {
                x: 11 - x,
                y: 11,
                direction: Up,
            },
            (4..8, 7) => State {
                x: 8,
                y: 11 - (x - 4),
                direction: Right,
            },
            (8..12, 11) => State {
                x: 3 - (x - 8),
                y: 7,
                direction: Up,
            },
            (12..16, 11) => State {
                x: 0,
                y: 7 - (x - 12),
                direction: Right,
            },
            _ => State {
                x,
                y: y + 1,
                direction,
            },
        },
        Up => match (x, y) {
            (0..4, 4) => State {
                x: 11 - x,
                y: 0,
                direction: Down,
            },
            (4..8, 4) => State {
                x: 8,
                y: x - 4,
                direction: Right,
            },
            (8..12, 0) => State {
                x: 4,
                y: 3 - (x - 8),
                direction: Down,
            },
            (12..16, 4) => State {
                x: 11,
                y: 7 - (x - 12),
                direction: Left,
            },
            _ => State {
                x,
                y: y - 1,
                direction,
            },
        },
    }
}
