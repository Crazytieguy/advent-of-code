use std::{cmp::Reverse, collections::BinaryHeap};

use ndarray::{Array, Array2, Axis};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> Array2<u32> {
    let lines: Vec<_> = data.lines().collect();
    Array::from_shape_vec(
        [lines.len(), lines[0].len()],
        data.chars().filter_map(|c| c.to_digit(10)).collect(),
    )
    .unwrap()
}

fn part_a(data: &'static str) -> u32 {
    let grid = parse(data);
    best_total_risk(&grid)
}

fn adjacent([x, y]: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
    [
        Some([x, y + 1]),
        Some([x + 1, y]),
        x.checked_sub(1).map(|x| [x, y]),
        y.checked_sub(1).map(|y| [x, y]),
    ]
    .into_iter()
    .flatten()
}

fn best_total_risk(grid: &Array2<u32>) -> u32 {
    let mut best_known = Array::from_elem(grid.shape(), u32::MAX);
    let mut queue = BinaryHeap::from([(Reverse(0), [0, 0])]);
    while let Some((Reverse(total_risk), idx)) = queue.pop() {
        let best_known_risk = best_known.get_mut(idx).unwrap();
        if total_risk < *best_known_risk {
            *best_known_risk = total_risk;
            for adj in adjacent(idx) {
                if let Some(risk) = grid.get(adj) {
                    queue.push((Reverse(total_risk + risk), adj));
                }
            }
        }
    }
    *best_known.last().unwrap()
}

fn cycle(risk: u32) -> u32 {
    (risk - 1) % 9 + 1
}

fn part_b(data: &'static str) -> u32 {
    let grid = parse(data);
    let mut five_grids = grid.clone();
    for i in 1..=4 {
        five_grids
            .append(Axis(0), grid.mapv(|r| cycle(r + i)).view())
            .unwrap()
    }
    let mut twent_five_grids = five_grids.clone();
    for i in 1..=4 {
        twent_five_grids
            .append(Axis(1), five_grids.mapv(|r| cycle(r + i)).view())
            .unwrap()
    }
    best_total_risk(&twent_five_grids)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 40);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 315);
    }
}
