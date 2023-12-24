use std::borrow::Cow;

use advent_2023::{BasicSolution, Solution};
use arrayvec::ArrayVec;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

struct Day;

type Coords = (u8, u8);
type BuildGraph = FxHashMap<Coords, FxHashMap<Coords, u16>>;
type FinalGraph = Vec<ArrayVec<(u8, u16), 4>>;

impl BasicSolution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Vec<&'static [u8]>;
    type Answer = u16;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 94;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 154;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        Ok(input.lines().map(str::as_bytes).collect())
    }

    fn part_a(grid: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        solve::<true>(&grid)
    }

    fn part_b(grid: Self::Shared) -> anyhow::Result<Self::Answer> {
        solve::<false>(&grid)
    }
}

fn solve<const PART_A: bool>(grid: &[&[u8]]) -> Result<u16, anyhow::Error> {
    let mut graph = BuildGraph::default();
    build_graph::<PART_A>(grid, &mut graph, (0, 1), (1, 1));
    let target_coords = (grid.len() as u8 - 1, grid[0].len() as u8 - 2);
    let (final_graph, start_node, target) = finalize_graph(graph, target_coords);
    let mut best = 0;
    let mut seen = vec![false; final_graph.len()];
    let total_bound = compute_total_bound(&final_graph);
    let traveled = 0;
    branch_and_bound(
        &final_graph,
        target,
        &mut best,
        &mut seen,
        total_bound,
        traveled,
        start_node,
    );
    Ok(best)
}

fn branch_and_bound(
    graph: &FinalGraph,
    target: u8,
    best: &mut u16,
    seen: &mut [bool],
    bound: u16,
    traveled: u16,
    node: u8,
) {
    seen[node as usize] = true;
    // After we chose a path, we can no longer benefit
    // from the length of the edges we didn't choose.
    let base_bound = bound
        - graph[node as usize]
            .iter()
            .filter(|&&(to, _)| !seen[to as usize])
            .map(|(_, length)| length)
            .sum::<u16>();
    for &(next_node, length) in &graph[node as usize] {
        if seen[next_node as usize] {
            continue;
        }
        let traveled = traveled + length;
        if next_node == target {
            *best = (*best).max(traveled);
            continue;
        }
        let bound = base_bound + length;
        if bound <= *best {
            continue;
        }
        branch_and_bound(graph, target, best, seen, bound, traveled, next_node);
    }
    seen[node as usize] = false;
}

fn compute_total_bound(graph: &FinalGraph) -> u16 {
    let mut counted = FxHashSet::default();
    let mut total_bound = 0;
    for (a, edges) in graph.iter().enumerate() {
        let a = a as u8;
        for (b, length) in edges {
            if counted.insert((a.min(*b), a.max(*b))) {
                total_bound += length;
            }
        }
    }
    total_bound
}

fn finalize_graph(graph: BuildGraph, target: Coords) -> (FinalGraph, u8, u8) {
    let mut coords_to_node = graph
        .keys()
        .enumerate()
        .map(|(node, &coords)| (coords, node as u8))
        .collect::<FxHashMap<Coords, u8>>();
    let start_node = coords_to_node[&(0, 1)];
    let target_node = *coords_to_node.entry(target).or_insert(graph.len() as u8);
    let mut final_graph = vec![ArrayVec::new(); coords_to_node.len()];
    for (a_coords, edges) in graph {
        let a_node = coords_to_node[&a_coords];
        for (b_coords, length) in edges {
            let b_node = coords_to_node[&b_coords];
            final_graph[a_node as usize].push((b_node, length));
        }
    }
    (final_graph, start_node, target_node)
}

fn build_graph<const PART_A: bool>(
    grid: &[&[u8]],
    graph: &mut BuildGraph,
    last_node: Coords,
    mut coords: Coords,
) {
    let mut prev_coords = last_node;
    for traveled in 1.. {
        if PART_A {
            if let Some(next_coords) = match get_2d(grid, coords) {
                Some(b'>') => Some((coords.0, coords.1 + 1)),
                Some(b'v') => Some((coords.0 + 1, coords.1)),
                Some(b'<') => Some((coords.0, coords.1 - 1)),
                Some(b'^') => Some((coords.0 - 1, coords.1)),
                _ => None,
            } {
                if next_coords == prev_coords {
                    return;
                }
                prev_coords = coords;
                coords = next_coords;
                continue;
            }
        }
        match adjacent_coords(coords)
            .filter(|&next_coords| {
                next_coords != prev_coords
                    && get_2d(grid, next_coords).is_some_and(|tile| tile != b'#')
            })
            .exactly_one()
        {
            Ok(next_coords) => {
                prev_coords = coords;
                coords = next_coords;
            }
            Err(multiple_coords) => {
                let is_new_node = !graph.contains_key(&coords);
                insert_edge(graph, last_node, coords, traveled);
                if PART_A {
                    // The previous node may or may not be reachable from the current node.
                    build_graph::<PART_A>(grid, graph, coords, prev_coords);
                } else {
                    // The previous node is always reachable from the current node.
                    insert_edge(graph, coords, last_node, traveled);
                }
                if is_new_node {
                    for next_coords in multiple_coords {
                        build_graph::<PART_A>(grid, graph, coords, next_coords)
                    }
                }
                return;
            }
        }
    }
}

fn insert_edge(graph: &mut BuildGraph, a: Coords, b: Coords, length: u16) {
    graph
        .entry(a)
        .or_default()
        .entry(b)
        .and_modify(|current| *current = (*current).max(length))
        .or_insert(length);
}

fn get_2d(grid: &[&[u8]], (row, col): Coords) -> Option<u8> {
    grid.get(row as usize)?.get(col as usize).copied()
}

fn adjacent_coords((row, col): Coords) -> impl Iterator<Item = Coords> {
    [(1, 0), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .flat_map(move |(drow, dcol)| {
            row.checked_add_signed(drow)
                .zip(col.checked_add_signed(dcol))
        })
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
