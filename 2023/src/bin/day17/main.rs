use std::{
    borrow::Cow,
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;

struct Day;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StateA {
    row: usize,
    col: usize,
    last_three_moves: [(isize, isize); 3],
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StateB {
    row: usize,
    col: usize,
    current_direction: (isize, isize),
    has_moved: usize,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");
    const SAMPLE_INPUT_B: &'static str = "111111111111
999999999991
999999999991
999999999991
999999999991
";

    type Shared = Vec<&'static [u8]>;
    type Answer = u32;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 102;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 71;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        Ok(input.lines().map(|line| line.as_bytes()).collect())
    }

    fn part_a(map: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        let initial_state = StateA::default();
        let mut queue = BinaryHeap::from([(Reverse(0), initial_state)]);
        let mut seen = HashSet::new();
        while let Some((Reverse(total_heat_loss), state)) = queue.pop() {
            if (state.row, state.col) == (map.len() - 1, map[0].len() - 1) {
                return Ok(total_heat_loss);
            }
            if !seen.insert(state.clone()) {
                continue;
            }
            for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if state.last_three_moves[2] == (-drow, -dcol) {
                    continue;
                }
                if state.last_three_moves == [(drow, dcol); 3] {
                    continue;
                }
                let Some(row) = state.row.checked_add_signed(drow) else {
                    continue;
                };
                let Some(col) = state.col.checked_add_signed(dcol) else {
                    continue;
                };
                let Some(heat_loss) = map.get(row).and_then(|row| row.get(col)) else {
                    continue;
                };
                let heat_loss = heat_loss - b'0';
                queue.push((
                    Reverse(total_heat_loss + heat_loss as u32),
                    StateA {
                        row,
                        col,
                        last_three_moves: [
                            state.last_three_moves[1],
                            state.last_three_moves[2],
                            (drow, dcol),
                        ],
                    },
                ))
            }
        }
        Err(anyhow!("No path found"))
    }

    fn part_b(map: Self::Shared) -> anyhow::Result<Self::Answer> {
        let initial_state = StateB {
            row: 0,
            col: 0,
            current_direction: (0, 0),
            has_moved: 4,
        };
        let mut queue = BinaryHeap::from([(Reverse(0), initial_state)]);
        let mut seen = HashSet::new();
        while let Some((Reverse(total_heat_loss), state)) = queue.pop() {
            if (state.row, state.col) == (map.len() - 1, map[0].len() - 1) {
                if state.has_moved < 4 {
                    continue;
                }
                return Ok(total_heat_loss);
            }
            if !seen.insert(state.clone()) {
                continue;
            }
            for (drow, dcol) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if state.current_direction == (-drow, -dcol) {
                    continue;
                }
                if state.has_moved < 4 && state.current_direction != (drow, dcol) {
                    continue;
                }
                if state.has_moved == 10 && state.current_direction == (drow, dcol) {
                    continue;
                }
                let Some(row) = state.row.checked_add_signed(drow) else {
                    continue;
                };
                let Some(col) = state.col.checked_add_signed(dcol) else {
                    continue;
                };
                let Some(heat_loss) = map.get(row).and_then(|row| row.get(col)) else {
                    continue;
                };
                let heat_loss = heat_loss - b'0';
                queue.push((
                    Reverse(total_heat_loss + heat_loss as u32),
                    StateB {
                        row,
                        col,
                        current_direction: (drow, dcol),
                        has_moved: if state.current_direction == (drow, dcol) {
                            state.has_moved + 1
                        } else {
                            1
                        },
                    },
                ))
            }
        }
        Err(anyhow!("No path found"))
    }
}

fn main() -> anyhow::Result<()> {
    Day::main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() -> anyhow::Result<()> {
        Day::test_part_a()
    }

    #[test]
    fn b() -> anyhow::Result<()> {
        Day::test_part_b()
    }
}
