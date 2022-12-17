use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{line_ending, u32},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};
use nom_supreme::ParserExt;
use petgraph::{algo::floyd_warshall, graph::NodeIndex, Graph};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap},
    error::Error,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = (
    Graph<u32, ()>,
    BTreeMap<(NodeIndex, NodeIndex), i32>,
    NodeIndex,
);

fn parse_row(data: &str) -> IResult<(&str, u32, Vec<&str>)> {
    tuple((
        tag("Valve ").precedes(take(2usize)),
        tag(" has flow rate=").precedes(u32),
        tag("; tunnels lead to valves ")
            .or(tag("; tunnel leads to valve "))
            .precedes(separated_list1(tag(", "), take(2usize))),
    ))(data)
}

fn parse(data: &str) -> IResult<Parsed> {
    let (input, rows) = separated_list1(line_ending, parse_row)(data)?;
    let mut graph = Graph::<u32, ()>::new();
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
    let starting_node = valve_name_to_idx["AA"];
    let shortest_paths = floyd_warshall(&graph, |_| 1).expect("no negative weights");
    let shortest_paths = shortest_paths
        .into_iter()
        .filter(|&((from, to), _)| (from == starting_node || graph[from] > 0) && graph[to] > 0)
        .collect();

    Ok((input, (graph, shortest_paths, starting_node)))
}

#[derive(Debug, Clone, Copy)]
enum NodeState {
    Unvisited,
    Visited,
    Visiting(u8),
}

#[derive(Debug, Clone, Copy)]
struct State<'a> {
    graph: &'a Graph<u32, ()>,
    shortest_paths: &'a BTreeMap<(NodeIndex, NodeIndex), i32>,
    elephant: Option<(usize, NodeState)>,
    node_states: [NodeState; 54],
    pressure_released: u32,
    minutes_remaining: u8,
}

impl<'a> State<'a> {
    fn update_pressure(&mut self, times: u32) {
        self.pressure_released += times
            * self
                .node_states
                .iter()
                .enumerate()
                .filter(|(_, &state)| matches!(state, NodeState::Visited))
                .map(|(i, _)| self.graph[NodeIndex::new(i)])
                .sum::<u32>();
    }

    fn bound(mut self) -> u32 {
        self.update_pressure(self.minutes_remaining as u32 + 1);
        // Assume that after visiting the next node, all valves will be opened one by one, sorted by weight descending
        let min_visiting = self
            .node_states
            .iter()
            .filter_map(|&node| {
                if let NodeState::Visiting(minutes) = node {
                    Some(minutes)
                } else {
                    None
                }
            })
            .min()
            .unwrap_or(0);
        let sorted_remaining_weights = self
            .node_states
            .iter()
            .enumerate()
            .filter_map(|(i, &node)| {
                if !matches!(node, NodeState::Visited) {
                    self.graph.node_weight(NodeIndex::new(i)).copied()
                } else {
                    None
                }
            })
            .sorted_unstable_by_key(|&w| Reverse(w))
            .collect_vec();

        for i in min_visiting..=self.minutes_remaining {
            for weight in sorted_remaining_weights.iter().take(i as usize + 1) {
                self.pressure_released += weight;
            }
        }

        self.pressure_released
    }

    fn solution(&self) -> Option<u32> {
        if !self.node_states.into_iter().any(|node| {
            if let NodeState::Visiting(minutes) = node {
                minutes < self.minutes_remaining
            } else {
                false
            }
        }) {
            let mut state = *self;
            state.update_pressure(state.minutes_remaining as u32 + 1);
            return Some(state.pressure_released);
        }
        None
    }

    fn branch(mut self) -> Vec<Self> {
        self.update_pressure(1);
        self.minutes_remaining -= 1;
        let mut branches = vec![self];
        branches[0].elephant = None;
        for (i, node_state) in self
            .node_states
            .into_iter()
            .enumerate()
            .chain(self.elephant)
        {
            if let NodeState::Visiting(minutes_remaining) = node_state {
                if minutes_remaining > 0 {
                    branches.iter_mut().for_each(|branch| {
                        branch.node_states[i] = NodeState::Visiting(minutes_remaining - 1);
                    });
                    continue;
                }
                branches = branches
                    .into_iter()
                    .flat_map(|mut branch| {
                        branch.node_states[i] = NodeState::Visited;
                        self.shortest_paths
                            .range(
                                (NodeIndex::new(i), NodeIndex::new(0))
                                    ..=(NodeIndex::new(i), NodeIndex::new(54)),
                            )
                            .map(|((_, destination), distance)| (destination, distance))
                            .filter(move |(destination, _)| {
                                !matches!(
                                    branch.node_states[destination.index()],
                                    NodeState::Visited
                                )
                            })
                            .map(move |(destination, distance)| {
                                let mut new_branch = branch;
                                if let NodeState::Visiting(cur_distance) =
                                    new_branch.node_states[destination.index()]
                                {
                                    if cur_distance <= *distance as u8 {
                                        return new_branch;
                                    }
                                }
                                new_branch.node_states[destination.index()] =
                                    NodeState::Visiting(*distance as u8);
                                new_branch
                            })
                            .chain([branch])
                    })
                    .collect();
            }
        }
        branches
    }
}

fn branch_and_bound(state: State, best: &mut u32) {
    if let Some(solution) = state.solution() {
        *best = solution.max(*best);
        return;
    }
    for branch in state.branch() {
        if branch.bound() > *best {
            branch_and_bound(branch, best);
        }
    }
}

fn part_a((graph, shortest_paths, starting_node): &Parsed) -> u32 {
    let mut best = 0;
    branch_and_bound(
        State {
            elephant: None,
            graph,
            shortest_paths,
            node_states: {
                let mut nodes = [NodeState::Unvisited; 54];
                nodes[starting_node.index()] = NodeState::Visiting(0);
                nodes
            },
            pressure_released: 0,
            minutes_remaining: 30,
        },
        &mut best,
    );
    best
}

fn part_b((graph, shortest_paths, starting_node): &Parsed) -> u32 {
    let mut best = 0;
    branch_and_bound(
        State {
            elephant: Some((starting_node.index(), NodeState::Visiting(0))),
            graph,
            shortest_paths,
            node_states: {
                let mut nodes = [NodeState::Unvisited; 54];
                nodes[starting_node.index()] = NodeState::Visiting(0);
                nodes
            },
            pressure_released: 0,
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
