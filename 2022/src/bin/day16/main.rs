use nom::{
    bytes::complete::{tag, take},
    character::complete::{line_ending, u8},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};
use nom_supreme::ParserExt;
use petgraph::{algo::floyd_warshall, graph::NodeIndex, Graph};
use std::{collections::HashMap, error::Error};
use tinyvec::ArrayVec;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type FlowRates = ArrayVec<[u8; 16]>;
type ShortestPathLengths = ArrayVec<[ArrayVec<[u8; 16]>; 16]>;

type Parsed = (FlowRates, ShortestPathLengths, usize);

fn parse_row(data: &str) -> IResult<(&str, u8, Vec<&str>)> {
    tuple((
        tag("Valve ").precedes(take(2usize)),
        tag(" has flow rate=").precedes(u8),
        tag("; tunnels lead to valves ")
            .or(tag("; tunnel leads to valve "))
            .precedes(separated_list1(tag(", "), take(2usize))),
    ))(data)
}

fn parse(data: &str) -> IResult<Parsed> {
    let (input, rows) = separated_list1(line_ending, parse_row)(data)?;
    let mut graph = Graph::<u8, ()>::new();
    let valve_name_to_idx: HashMap<&str, _> = rows
        .iter()
        .map(|&(name, flow, _)| (name, graph.add_node(flow)))
        .collect();
    for (name, _, leads_to) in rows {
        let from = valve_name_to_idx[name];
        for to in leads_to {
            graph.add_edge(from, valve_name_to_idx[to], ());
        }
    }
    let starting_valve = valve_name_to_idx["AA"];
    let shortest_paths_map = floyd_warshall(&graph, |_| 1).expect("no negative weights");
    let graph_indexes_to_regular_indexes: HashMap<NodeIndex, usize> = graph
        .node_indices()
        .filter(|&i| i == starting_valve || graph[i] > 0)
        .enumerate()
        .map(|(i, idx)| (idx, i))
        .collect();

    let mut flow_rates = ArrayVec::from_array_len([0; 16], graph_indexes_to_regular_indexes.len());
    for (&idx, &i) in &graph_indexes_to_regular_indexes {
        flow_rates[i] = graph[idx];
    }
    let mut shortest_path_lengths = ArrayVec::from_array_len(
        [ArrayVec::from_array_len([0; 16], graph_indexes_to_regular_indexes.len()); 16],
        graph_indexes_to_regular_indexes.len(),
    );

    for ((from, to), distance) in shortest_paths_map {
        if let (Some(from), Some(to)) = (
            graph_indexes_to_regular_indexes.get(&from),
            graph_indexes_to_regular_indexes.get(&to),
        ) {
            shortest_path_lengths[*from][*to] = distance;
        }
    }

    Ok((
        input,
        (
            flow_rates,
            shortest_path_lengths,
            graph_indexes_to_regular_indexes[&starting_valve],
        ),
    ))
}

#[derive(Default, Debug, Clone, Copy)]
struct VisitorState {
    going_to: usize,
    will_arrive_in: u8,
}

#[derive(Default, Debug, Clone, Copy)]
struct State {
    visitors: ArrayVec<[VisitorState; 2]>,
    visited: ArrayVec<[bool; 16]>,
    pressure_released: u16,
    current_flow: u16,
    minutes_remaining: u8,
}

/// Attempt to follow the 'branch and bound' algorithm from wikipedia:
/// https://en.wikipedia.org/wiki/Branch_and_bound
impl State {
    fn solution(&self) -> Option<u16> {
        if self
            .visitors
            .iter()
            .any(|v| v.will_arrive_in < self.minutes_remaining)
        {
            None
        } else {
            Some(self.pressure_released + (self.minutes_remaining as u16 + 1) * self.current_flow)
        }
    }

    /// Assuming the shortest path lengths are all 1, the best solution is
    /// to visit the valves in order of descending flow rate.
    fn bound(mut self, flow_rates: &FlowRates) -> u16 {
        let mut remaining_flow_rate_indices: ArrayVec<[usize; 16]> = (0..flow_rates.len())
            .filter(|&i| !self.visited[i] && !self.visitors.iter().any(|v| v.going_to == i))
            .collect();
        remaining_flow_rate_indices.sort_unstable_by_key(|&i| flow_rates[i]);
        while self.minutes_remaining > 0 {
            self.minutes_remaining -= 1;
            self.pressure_released += self.current_flow;
            for visitor in self.visitors.iter_mut() {
                if visitor.will_arrive_in > 0 {
                    visitor.will_arrive_in -= 1;
                    continue;
                }
                self.current_flow += flow_rates[visitor.going_to] as u16;
                if let Some(i) = remaining_flow_rate_indices.pop() {
                    visitor.going_to = i;
                    visitor.will_arrive_in = 1;
                } else {
                    visitor.will_arrive_in = u8::MAX;
                }
            }
        }
        self.pressure_released + self.current_flow
    }

    /// Each visitor that has reached their destination multiplies
    /// the amount of new states by the number of valves that haven't been visited
    fn branch(
        mut self,
        flow_rates: &FlowRates,
        shortest_path_lengths: &ShortestPathLengths,
    ) -> impl IntoIterator<Item = Self> {
        self.pressure_released += self.current_flow;
        self.minutes_remaining -= 1;
        let mut branches = vec![self];
        for (visitor_idx, visitor) in self.visitors.into_iter().enumerate() {
            if visitor.will_arrive_in > 0 {
                branches
                    .iter_mut()
                    .for_each(|state| state.visitors[visitor_idx].will_arrive_in -= 1);
                continue;
            }
            branches.iter_mut().for_each(|state| {
                state.visited[visitor.going_to] = true;
                state.current_flow += flow_rates[visitor.going_to] as u16;
            });
            branches = branches
                .iter()
                .flat_map(|&state| {
                    shortest_path_lengths[visitor.going_to]
                        .iter()
                        .enumerate()
                        .filter(move |&(destination, _)| {
                            !state.visited[destination]
                                && !state.visitors.iter().any(|v| v.going_to == destination)
                        })
                        .map(move |(destination, &distance)| {
                            let mut next_state = state;
                            next_state.visitors[visitor_idx].going_to = destination;
                            next_state.visitors[visitor_idx].will_arrive_in = distance;
                            next_state
                        })
                })
                .chain(
                    // in case there's no where left to visit
                    [{
                        let mut state = branches[0];
                        state.visitors[visitor_idx].will_arrive_in = u8::MAX;
                        state
                    }],
                )
                .collect();
        }
        branches
    }
}

fn branch_and_bound(
    flow_rates: &FlowRates,
    shortest_path_lengths: &ShortestPathLengths,
    state: State,
    best: &mut u16,
) {
    if let Some(solution) = state.solution() {
        *best = solution.max(*best);
        return;
    }
    for branch in state.branch(flow_rates, shortest_path_lengths) {
        if branch.bound(flow_rates) > *best {
            branch_and_bound(flow_rates, shortest_path_lengths, branch, best);
        }
    }
}

fn part_a((flow_rates, shortest_paths, starting_idx): &Parsed) -> u16 {
    let mut best = 0;
    branch_and_bound(
        flow_rates,
        shortest_paths,
        State {
            visitors: ArrayVec::from_array_len(
                [VisitorState {
                    going_to: *starting_idx,
                    will_arrive_in: 0,
                }; 2],
                1,
            ),
            visited: ArrayVec::from_array_len([false; 16], flow_rates.len()),
            pressure_released: 0,
            current_flow: 0,
            minutes_remaining: 30,
        },
        &mut best,
    );
    best
}

fn part_b((flow_rates, shortest_paths, starting_idx): &Parsed) -> u16 {
    let mut best = 0;
    branch_and_bound(
        flow_rates,
        shortest_paths,
        State {
            visitors: ArrayVec::from_array_len(
                [VisitorState {
                    going_to: *starting_idx,
                    will_arrive_in: 0,
                }; 2],
                2,
            ),
            visited: ArrayVec::from_array_len([false; 16], flow_rates.len()),
            pressure_released: 0,
            current_flow: 0,
            minutes_remaining: 26,
        },
        &mut best,
    );
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 1651);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 1707);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
