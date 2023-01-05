use std::{collections::HashSet, error::Error};

use itertools::Itertools;
use Direction::*;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: u8,
    col: u8,
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    direction: Direction,
    position: Position,
}

fn parse(data: &str) -> (Vec<Blizzard>, Position) {
    let lines = data.lines().collect_vec();
    let height = lines.len() as u8;
    let width = lines[0].len() as u8;
    let blizzards = lines
        .into_iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                let direction = match c {
                    '>' => Right,
                    '<' => Left,
                    'v' => Down,
                    '^' => Up,
                    _ => return None,
                };
                Some(Blizzard {
                    direction,
                    position: Position {
                        row: row as u8,
                        col: col as u8,
                    },
                })
            })
        })
        .collect();
    (
        blizzards,
        Position {
            row: height,
            col: width,
        },
    )
}

impl Position {
    fn checked_right(self, bounds: Position) -> Option<Self> {
        if self.col == bounds.col - 2 || self.row == 0 {
            None
        } else {
            Some(Position {
                col: self.col + 1,
                ..self
            })
        }
    }

    fn wrapping_right(self, bounds: Position) -> Self {
        self.checked_right(bounds)
            .unwrap_or(Position { col: 1, ..self })
    }

    fn checked_left(self, bounds: Position) -> Option<Self> {
        if self.col == 1 || self.row == bounds.row - 1 {
            None
        } else {
            Some(Position {
                col: self.col - 1,
                ..self
            })
        }
    }

    fn wrapping_left(self, bounds: Position) -> Self {
        self.checked_left(bounds).unwrap_or(Position {
            col: bounds.col - 2,
            ..self
        })
    }

    fn checked_up(self, _bounds: Position) -> Option<Self> {
        match self.row {
            0 => None,
            1 if self.col == 1 => Some(Position { row: 0, ..self }),
            1 => None,
            _ => Some(Position {
                row: self.row - 1,
                ..self
            }),
        }
    }

    fn wrapping_up(self, bounds: Position) -> Self {
        self.checked_up(bounds).unwrap_or(Position {
            row: bounds.row - 2,
            ..self
        })
    }

    fn checked_down(self, bounds: Position) -> Option<Self> {
        if self.row == bounds.row - 1 {
            None
        } else if self.row == bounds.row - 2 {
            if self.col == bounds.col - 2 {
                Some(Position {
                    row: bounds.row - 1,
                    ..self
                })
            } else {
                None
            }
        } else {
            Some(Position {
                row: self.row + 1,
                ..self
            })
        }
    }

    fn wrapping_down(self, bounds: Position) -> Self {
        self.checked_down(bounds)
            .unwrap_or(Position { row: 1, ..self })
    }
}

fn simulate_shortest_path(
    blizzards: &mut [Blizzard],
    &bounds: &Position,
    start: Position,
    goal: Position,
) -> usize {
    let mut positions = HashSet::from([start]);
    for minute in 1.. {
        positions = positions
            .into_iter()
            .flat_map(|p| {
                [
                    Some(p),
                    p.checked_down(bounds),
                    p.checked_up(bounds),
                    p.checked_left(bounds),
                    p.checked_right(bounds),
                ]
            })
            .flatten()
            .collect();
        for blizzard in blizzards.iter_mut() {
            blizzard.position = match blizzard.direction {
                Up => blizzard.position.wrapping_up(bounds),
                Down => blizzard.position.wrapping_down(bounds),
                Left => blizzard.position.wrapping_left(bounds),
                Right => blizzard.position.wrapping_right(bounds),
            };
            positions.remove(&blizzard.position);
        }
        if positions.contains(&goal) {
            return minute;
        }
    }
    unreachable!()
}

fn part_a((blizzards, bounds): &(Vec<Blizzard>, Position)) -> usize {
    let mut blizzards = blizzards.clone();
    let start = Position { row: 0, col: 1 };
    let goal = Position {
        row: bounds.row - 1,
        col: bounds.col - 2,
    };
    simulate_shortest_path(&mut blizzards, bounds, start, goal)
}

fn part_b((blizzards, bounds): &(Vec<Blizzard>, Position)) -> usize {
    let mut blizzards = blizzards.clone();
    let start = Position { row: 0, col: 1 };
    let goal = Position {
        row: bounds.row - 1,
        col: bounds.col - 2,
    };
    simulate_shortest_path(&mut blizzards, bounds, start, goal)
        + simulate_shortest_path(&mut blizzards, bounds, goal, start)
        + simulate_shortest_path(&mut blizzards, bounds, start, goal)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 18);
        println!("part a: {}", part_a(&parse(DATA)));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 54);
        println!("part b: {}", part_b(&parse(DATA)));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
