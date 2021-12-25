use itertools::Itertools;
use ndarray::{Array, Array2};
use SeaCucumber::*;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SeaCucumber {
    East,
    South,
}

fn parse(data: &'static str) -> Array2<Option<SeaCucumber>> {
    let lines: Vec<_> = data.lines().collect();
    Array::from_shape_vec(
        [lines.len(), lines[0].len()],
        data.chars()
            .filter_map(|c| match c {
                '>' => Some(Some(East)),
                'v' => Some(Some(South)),
                '.' => Some(None),
                _ => None,
            })
            .collect(),
    )
    .unwrap()
}

// For debugging
#[allow(dead_code)]
fn print_grid(grid: &Array2<Option<SeaCucumber>>) {
    let s: String = grid
        .outer_iter()
        .flat_map(|row| {
            row.iter()
                .map(|c| match c {
                    Some(East) => '>',
                    Some(South) => 'v',
                    None => '.',
                })
                .chain(Some('\n'))
                .collect_vec()
        })
        .collect();
    println!("{}", s)
}

fn step_direction(grid: &mut Array2<Option<SeaCucumber>>, direction: SeaCucumber) -> bool {
    let lanes = match direction {
        East => grid.rows_mut(),
        South => grid.columns_mut(),
    };
    let mut moved = false;
    let mut indexes_to_move = Vec::with_capacity(140);
    lanes.into_iter().for_each(|mut lane| {
        indexes_to_move.clear();
        indexes_to_move.extend(
            lane.windows(2)
                .into_iter()
                .enumerate()
                .filter(|(_, pair)| [pair[0], pair[1]] == [Some(direction), None])
                .map(|(i, _)| i),
        );
        if [*lane.last().unwrap(), lane[0]] == [Some(direction), None] {
            moved = true;
            *lane.last_mut().unwrap() = None;
            lane[0] = Some(direction);
        }
        if !indexes_to_move.is_empty() {
            moved = true
        }
        for &i in &indexes_to_move {
            lane[i] = None;
            lane[i + 1] = Some(direction);
        }
    });
    moved
}

fn step_both_directions(grid: &mut Array2<Option<SeaCucumber>>) -> bool {
    let mut moved = step_direction(grid, East);
    moved |= step_direction(grid, South);
    moved
}

fn part_a(data: &'static str) -> usize {
    let mut grid = parse(data);
    for i in 1.. {
        if !step_both_directions(&mut grid) {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 58);
    }
}
