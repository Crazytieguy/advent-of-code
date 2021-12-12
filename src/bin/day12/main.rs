use std::{collections::HashMap, iter};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");
const MAX_CAVES: usize = 11;

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

struct Graph {
    start: usize,
    end: usize,
    is_small: [bool; MAX_CAVES],
    connected_to: [Vec<usize>; MAX_CAVES],
}

fn parse(data: &'static str) -> Graph {
    let mut is_small = [false; MAX_CAVES];
    let mut connected_to: [Vec<usize>; MAX_CAVES] = Default::default();
    let str_graph = data
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split('-').collect_tuple().unwrap();
            [(a, b), (b, a)]
        })
        .filter(|&(a, b)| a != "end" && b != "start")
        .into_group_map();
    let mut name_to_id = HashMap::new();
    for (id, &name) in str_graph.keys().chain(iter::once(&"end")).enumerate() {
        is_small[id] = name.chars().next().unwrap().is_ascii_lowercase();
        name_to_id.insert(name, id);
    }
    for (from, to) in str_graph {
        connected_to[name_to_id[from]] = to.into_iter().map(|name| name_to_id[name]).collect()
    }
    Graph {
        start: name_to_id["start"],
        end: name_to_id["end"],
        is_small,
        connected_to,
    }
}

fn count_paths(
    graph: &Graph,
    cur_position: usize,
    mut visited: [bool; MAX_CAVES],
    second_visit_allowed: bool,
) -> usize {
    if cur_position == graph.end {
        return 1;
    }
    visited[cur_position] = graph.is_small[cur_position];
    graph.connected_to[cur_position]
        .iter()
        .filter(|&&next_position| second_visit_allowed || !visited[next_position])
        .map(|&next_position| {
            count_paths(
                graph,
                next_position,
                visited,
                second_visit_allowed && !visited[next_position],
            )
        })
        .sum()
}

fn part_a(data: &'static str) -> usize {
    let graph = parse(data);
    count_paths(&graph, graph.start, Default::default(), false)
}

fn part_b(data: &'static str) -> usize {
    let graph = parse(data);
    count_paths(&graph, graph.start, Default::default(), true)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 226);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 3509);
    }
}
