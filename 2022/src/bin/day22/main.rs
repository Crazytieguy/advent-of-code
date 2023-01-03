#![feature(exclusive_range_pattern)]
use std::error::Error;

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, u32},
    multi::many1,
    Parser,
};
use nom_supreme::ParserExt;
use Direction::*;
use Move::*;
use Rotation::*;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed<'a> = (Vec<&'a [u8]>, Vec<Move>);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Turn(Rotation),
    Forward(usize),
}

fn parse_move(input: &str) -> IResult<Move> {
    alt((
        char('R').value(Turn(Clockwise)),
        char('L').value(Turn(CounterClockwise)),
        u32.map(|n| Forward(n as usize)),
    ))(input)
}

fn parse(data: &str) -> IResult<Parsed> {
    let mut lines = data.lines();
    let grid = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(str::as_bytes)
        .collect_vec();
    let path = lines
        .next()
        .expect("there should be a path after the empty line");
    many1(parse_move)
        .parse(path)
        .map(|(rest, moves)| (rest, (grid, moves)))
}

fn down(grid: &[&[u8]], x: usize, y: usize, dy: usize) -> usize {
    grid.iter()
        .enumerate()
        .cycle()
        .skip(y)
        .filter(|&(_, row)| row.get(x).filter(|&&c| c != b' ').is_some())
        .take(dy + 1)
        .take_while(|&(_, row)| row[x] != b'#')
        .last()
        .unwrap()
        .0
}

fn up(grid: &[&[u8]], x: usize, y: usize, dy: usize) -> usize {
    grid.iter()
        .enumerate()
        .rev()
        .cycle()
        .skip(grid.len() - y - 1)
        .filter(|&(_, row)| row.get(x).filter(|&&c| c != b' ').is_some())
        .take(dy + 1)
        .take_while(|&(_, row)| row[x] != b'#')
        .last()
        .unwrap()
        .0
}

fn right(grid: &[&[u8]], x: usize, y: usize, dx: usize) -> usize {
    grid[y]
        .iter()
        .enumerate()
        .cycle()
        .skip(x)
        .filter(|&(_, &c)| c != b' ')
        .take(dx + 1)
        .take_while(|&(_, &c)| c != b'#')
        .last()
        .unwrap()
        .0
}

fn left(grid: &[&[u8]], x: usize, y: usize, dx: usize) -> usize {
    grid[y]
        .iter()
        .enumerate()
        .rev()
        .cycle()
        .skip(grid[y].len() - x - 1)
        .filter(|&(_, &c)| c != b' ')
        .take(dx + 1)
        .take_while(|&(_, &c)| c != b'#')
        .last()
        .unwrap()
        .0
}

fn part_a((grid, path): &Parsed) -> usize {
    let mut y = 0;
    let mut x = grid[0].iter().position(|&c| c == b'.').unwrap();
    let mut direction = Right;
    for &m in path {
        match m {
            Turn(Clockwise) => {
                direction = match direction {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                }
            }
            Turn(CounterClockwise) => {
                direction = match direction {
                    Up => Left,
                    Left => Down,
                    Down => Right,
                    Right => Up,
                }
            }
            Forward(n) => match direction {
                Up => y = up(grid, x, y, n),
                Down => y = down(grid, x, y, n),
                Left => x = left(grid, x, y, n),
                Right => x = right(grid, x, y, n),
            },
        }
    }
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

#[derive(Debug, Clone, Copy)]
struct State {
    x: usize,
    y: usize,
    direction: Direction,
}

fn move_one_cube(State { x, y, direction }: State) -> State {
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

fn part_b((grid, path): &Parsed, move_one: fn(State) -> State) -> usize {
    let mut state = State {
        y: 0,
        x: grid[0].iter().position(|&c| c == b'.').unwrap(),
        direction: Right,
    };
    for &m in path {
        match m {
            Turn(Clockwise) => {
                state.direction = match state.direction {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                }
            }
            Turn(CounterClockwise) => {
                state.direction = match state.direction {
                    Up => Left,
                    Left => Down,
                    Down => Right,
                    Right => Up,
                }
            }
            Forward(n) => {
                for _ in 0..n {
                    let next_state = move_one(state);
                    if grid[next_state.y][next_state.x] == b'#' {
                        break;
                    }
                    state = next_state;
                }
            }
        }
    }
    let row_number = state.y + 1;
    let column_number = state.x + 1;
    let facing_number = match state.direction {
        Right => 0,
        Down => 1,
        Left => 2,
        Up => 3,
    };
    1000 * row_number + 4 * column_number + facing_number
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    fn move_one_sample_cube(State { x, y, direction }: State) -> State {
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

    #[test]
    #[ignore]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 6032);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1, move_one_sample_cube), 5031);
        println!("part b: {}", part_b(&parse(DATA)?.1, move_one_cube));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed, move_one_cube));
    Ok(())
}
