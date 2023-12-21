use std::{borrow::Cow, cmp::Reverse, collections::BinaryHeap};

use advent_2023::{BasicSolution, Solution};
use anyhow::anyhow;
use fxhash::FxHashSet;

struct Day;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    row: u8,
    col: u8,
    current_direction: (i8, i8),
    has_moved: u8,
}

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample_a.txt");
    const SAMPLE_INPUT_B: &'static str = include_str!("sample_b.txt");

    type Shared = Vec<&'static [u8]>;
    type Answer = u16;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 102;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 71;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        Ok(input.lines().map(|line| line.as_bytes()).collect())
    }

    fn part_a(map: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        let initial_state = State {
            row: 0,
            col: 0,
            current_direction: (0, 0),
            has_moved: 0,
        };
        djkstra(
            &map,
            initial_state,
            |state, (drow, dcol)| state.has_moved == 3 && state.current_direction == (drow, dcol),
            |state| (state.row as usize, state.col as usize) == (map.len() - 1, map[0].len() - 1),
        )
    }

    fn part_b(map: Self::Shared) -> anyhow::Result<Self::Answer> {
        let initial_state = State {
            row: 0,
            col: 0,
            current_direction: (0, 0),
            has_moved: 4,
        };
        djkstra(
            &map,
            initial_state,
            |state, (drow, dcol)| {
                (state.has_moved < 4 && state.current_direction != (drow, dcol))
                    || (state.has_moved == 10 && state.current_direction == (drow, dcol))
            },
            |state| {
                state.has_moved >= 4
                    && (state.row as usize, state.col as usize) == (map.len() - 1, map[0].len() - 1)
            },
        )
    }
}

fn djkstra(
    map: &[&[u8]],
    initial_state: State,
    invalid_move: impl Fn(&State, (i8, i8)) -> bool,
    target: impl Fn(&State) -> bool,
) -> Result<u16, anyhow::Error> {
    let mut queue = BinaryHeap::from([(Reverse(0), initial_state)]);
    let mut seen = FxHashSet::default();
    while let Some((Reverse(total_heat_loss), state)) = queue.pop() {
        if target(&state) {
            return Ok(total_heat_loss);
        }
        if !seen.insert(state.clone()) {
            continue;
        }
        for (drow, dcol) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if state.current_direction == (-drow, -dcol) {
                continue;
            }
            if invalid_move(&state, (drow, dcol)) {
                continue;
            }
            let Some(row) = state.row.checked_add_signed(drow) else {
                continue;
            };
            let Some(col) = state.col.checked_add_signed(dcol) else {
                continue;
            };
            let Some(heat_loss) = map.get(row as usize).and_then(|row| row.get(col as usize))
            else {
                continue;
            };
            let heat_loss = heat_loss - b'0';
            let has_moved = if state.current_direction == (drow, dcol) {
                state.has_moved + 1
            } else {
                1
            };
            queue.push((
                Reverse(total_heat_loss + heat_loss as u16),
                State {
                    row,
                    col,
                    current_direction: (drow, dcol),
                    has_moved,
                },
            ))
        }
    }
    Err(anyhow!("No path found"))
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
