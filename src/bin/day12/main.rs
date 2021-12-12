use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> HashMap<&str, Vec<&str>> {
    data.lines()
        .flat_map(|line| {
            let (a, b) = line.split('-').collect_tuple().unwrap();
            [(a, b), (b, a)]
        })
        .filter(|&(a, b)| a != "end" && b != "start")
        .into_group_map()
}

fn count_paths<'a>(
    graph: &HashMap<&'a str, Vec<&'a str>>,
    cur_position: &'a str,
    mut visited: HashSet<&'a str>,
    second_visit_allowed: bool,
) -> usize {
    if cur_position == "end" {
        return 1;
    }
    if cur_position.chars().all(|c| c.is_ascii_lowercase()) {
        visited.insert(cur_position);
    };
    graph[cur_position]
        .iter()
        .filter(|&&next_position| second_visit_allowed || !visited.contains(next_position))
        .map(|next_position| {
            count_paths(
                graph,
                next_position,
                visited.clone(),
                second_visit_allowed && !visited.contains(next_position),
            )
        })
        .sum()
}

fn part_a(data: &'static str) -> usize {
    let graph = parse(data);
    count_paths(&graph, "start", HashSet::new(), false)
}

fn part_b(data: &'static str) -> usize {
    let graph = parse(data);
    count_paths(&graph, "start", HashSet::new(), true)
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
