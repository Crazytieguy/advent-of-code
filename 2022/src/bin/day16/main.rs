use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{line_ending, u8},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};
use nom_supreme::ParserExt;
use std::{cmp::Reverse, collections::HashMap, error::Error};
use tinyvec::ArrayVec;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type FlowRates = Vec<u8>;
type ShortestPathLengths = Vec<Vec<u8>>;

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

// simplified copy of petgraph's implementation
fn floyd_warshall(rows: &[(&str, u8, Vec<&str>)]) -> Vec<Vec<u8>> {
    let valve_name_to_idx: HashMap<&str, _> = rows
        .iter()
        .enumerate()
        .map(|(i, &(name, _, _))| (name, i))
        .collect();

    let mut dist = vec![vec![u8::MAX; rows.len()]; rows.len()];
    for (i, (_, _, tunnels)) in rows.iter().enumerate() {
        for tunnel in tunnels {
            let j = valve_name_to_idx[tunnel];
            dist[i][j] = 1;
        }
    }
    (0..dist.len()).for_each(|i| {
        dist[i][i] = 0;
    });
    for k in 0..dist.len() {
        for i in 0..dist.len() {
            for j in 0..dist.len() {
                let (result, overflow) = dist[i][k].overflowing_add(dist[k][j]);
                if !overflow && dist[i][j] > result {
                    dist[i][j] = result;
                }
            }
        }
    }
    dist
}

fn parse(data: &str) -> IResult<Parsed> {
    let (input, rows) = separated_list1(line_ending, parse_row)(data)?;
    let shortest_path_lengths_uncompressed = floyd_warshall(&rows);

    let interesting_valve_indices = rows
        .iter()
        .enumerate()
        .filter(|&(_, &(name, flow, _))| name == "AA" || flow > 0)
        .map(|(i, _)| i)
        .collect_vec();

    let flow_rates = interesting_valve_indices
        .iter()
        .map(|&i| rows[i].1)
        .collect();

    let shortest_path_lengths = interesting_valve_indices
        .iter()
        .map(|&i| {
            interesting_valve_indices
                .iter()
                .map(|&j| shortest_path_lengths_uncompressed[i][j])
                .collect()
        })
        .collect();

    let starting_node = interesting_valve_indices
        .iter()
        .position(|&i| rows[i].0 == "AA")
        .expect("a valve called AA");

    Ok((input, (flow_rates, shortest_path_lengths, starting_node)))
}

#[derive(Default, Debug, Clone, Copy)]
struct VisitorState {
    going_to: u8,
    will_arrive_in: u8,
}

#[derive(Default, Debug, Clone, Copy)]
struct State {
    visitors: [VisitorState; 2],
    visited: u16,
    pressure_released: u16,
    current_flow: u16,
    minutes_remaining: u8,
}

/// Attempt to follow the 'branch and bound' algorithm from wikipedia:
/// https://en.wikipedia.org/wiki/Branch_and_bound
impl State {
    fn visited(&self, i: usize) -> bool {
        self.visited & (1 << i) != 0
    }

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
            .filter(|&i| !self.visited(i) && !self.visitors.iter().any(|v| v.going_to == i as u8))
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
                self.current_flow += flow_rates[visitor.going_to as usize] as u16;
                if let Some(i) = remaining_flow_rate_indices.pop() {
                    visitor.going_to = i as u8;
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
                state.visited |= 1 << visitor.going_to;
                state.current_flow += flow_rates[visitor.going_to as usize] as u16;
            });
            branches = branches
                .iter()
                .flat_map(|&state| {
                    shortest_path_lengths[visitor.going_to as usize]
                        .iter()
                        .enumerate()
                        .filter(move |&(destination, _)| {
                            !state.visited(destination)
                                && !state
                                    .visitors
                                    .iter()
                                    .any(|v| v.going_to == destination as u8)
                        })
                        .map(move |(destination, &distance)| {
                            let mut next_state = state;
                            next_state.visitors[visitor_idx].going_to = destination as u8;
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
    let bound_branch_pairs = state
        .branch(flow_rates, shortest_path_lengths)
        .into_iter()
        .map(|state| (state.bound(flow_rates), state))
        .filter(|(bound, _)| bound > best)
        .sorted_unstable_by_key(|(bound, _)| Reverse(*bound))
        .collect_vec();
    for (bound, branch) in bound_branch_pairs {
        if bound > *best {
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
            visitors: [
                VisitorState {
                    going_to: *starting_idx as u8,
                    will_arrive_in: 0,
                },
                VisitorState {
                    going_to: *starting_idx as u8,
                    will_arrive_in: u8::MAX,
                },
            ],
            visited: 0,
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
            visitors: [
                VisitorState {
                    going_to: *starting_idx as u8,
                    will_arrive_in: 0,
                },
                VisitorState {
                    going_to: *starting_idx as u8,
                    will_arrive_in: 0,
                },
            ],
            visited: 0,
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
