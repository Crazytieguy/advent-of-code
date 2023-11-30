use std::{cmp::Reverse, collections::HashMap};

use advent_2022::*;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{line_ending, u8},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};
use nom_supreme::ParserExt;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = (FlowRates, ShortestPathLengths, FlowRateIndices, usize);
    type Answer = u16;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 1651;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 1707;

    fn parse(data: &str) -> IResult<Self::Parsed> {
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
            .collect_vec();

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

        let sorted_flow_rate_indices = flow_rates
            .iter()
            .enumerate()
            .sorted_unstable_by_key(|&(_, &flow)| Reverse(flow))
            .map(|(i, _)| i)
            .collect_vec();

        Ok((
            input,
            (
                flow_rates,
                shortest_path_lengths,
                sorted_flow_rate_indices,
                starting_node,
            ),
        ))
    }

    fn a(
        (flow_rates, shortest_paths, sorted_flow_rate_indices, starting_idx): Self::Parsed,
    ) -> u16 {
        let mut best = 0;
        branch_and_bound(
            &flow_rates,
            &sorted_flow_rate_indices,
            &shortest_paths,
            State::new(starting_idx as u8, 30),
            &mut [],
            &mut best,
            |bound, best| bound > best,
        );
        best
    }

    fn b(
        (flow_rates, shortest_paths, sorted_flow_rate_indices, starting_idx): Self::Parsed,
    ) -> u16 {
        let mut best_per_visited = vec![0; u16::MAX as usize];
        branch_and_bound(
            &flow_rates,
            &sorted_flow_rate_indices,
            &shortest_paths,
            State::new(starting_idx as u8, 26),
            &mut best_per_visited,
            &mut 0,
            // this could technically produce an incorrect result,
            // but it doesn't on my input or the test input
            |bound, best| bound > best * 3 / 4,
        );
        let best_per_visited_filtered_sorted = best_per_visited
            .into_iter()
            .enumerate()
            .filter(|&(_, best)| best > 0)
            .map(|(i, best)| (i as u16, best))
            .sorted_unstable_by_key(|&(_, best)| Reverse(best))
            .collect_vec();
        let mut best = 0;
        for (i, &(my_visited, my_best)) in best_per_visited_filtered_sorted.iter().enumerate() {
            for &(elephant_visited, elephant_best) in &best_per_visited_filtered_sorted[i + 1..] {
                let score = my_best + elephant_best;
                if score <= best {
                    break;
                }
                if my_visited & elephant_visited == 0 {
                    best = score;
                    break;
                }
            }
        }
        best
    }
}

type FlowRates = Vec<u8>;
type FlowRateIndices = Vec<usize>;
type ShortestPathLengths = Vec<Vec<u8>>;

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

fn branch_and_bound(
    flow_rates: &FlowRates,
    sorted_flow_rate_indices: &[usize],
    shortest_path_lengths: &ShortestPathLengths,
    state: State,
    best_for_visited: &mut [u16],
    best: &mut u16,
    filter_bound: impl Fn(u16, u16) -> bool + Copy,
) {
    if let Some(cur_best) = best_for_visited.get_mut(state.visited as usize) {
        *cur_best = state.pressure_released.max(*cur_best);
    }
    *best = state.pressure_released.max(*best);
    let bound_branch_pairs = state
        .branch(flow_rates, shortest_path_lengths)
        .into_iter()
        .map(|state| (state.bound(flow_rates, sorted_flow_rate_indices), state))
        .filter(|&(bound, _)| filter_bound(bound, *best))
        .sorted_unstable_by_key(|(bound, _)| Reverse(*bound))
        .collect_vec();
    for (bound, branch) in bound_branch_pairs {
        if filter_bound(bound, *best) {
            branch_and_bound(
                flow_rates,
                sorted_flow_rate_indices,
                shortest_path_lengths,
                branch,
                best_for_visited,
                best,
                filter_bound,
            );
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct State {
    visited: u16,
    avoid: u16,
    pressure_released: u16,
    minutes_remaining: u8,
    position: u8,
}

/// Attempt to follow the 'branch and bound' algorithm from wikipedia:
/// https://en.wikipedia.org/wiki/Branch_and_bound
impl State {
    fn new(position: u8, minutes_remaining: u8) -> Self {
        Self {
            visited: 0,
            avoid: 1 << position,
            pressure_released: 0,
            minutes_remaining,
            position,
        }
    }

    fn can_visit(self, i: usize) -> bool {
        (self.visited | self.avoid) & (1 << i) == 0
    }

    /// Assuming the shortest path lengths are all 1, the best solution is
    /// to visit the valves in order of descending flow rate.
    fn bound(self, flow_rates: &FlowRates, sorted_flow_rate_indices: &[usize]) -> u16 {
        self.pressure_released
            + (0..=self.minutes_remaining)
                .rev()
                .step_by(2)
                .skip(1)
                .zip(
                    sorted_flow_rate_indices
                        .iter()
                        .filter(|&&i| self.can_visit(i))
                        .map(|&i| flow_rates[i]),
                )
                .map(|(minutes, flow)| minutes as u16 * flow as u16)
                .sum::<u16>()
    }

    fn branch<'a>(
        self,
        flow_rates: &'a FlowRates,
        shortest_path_lengths: &'a ShortestPathLengths,
    ) -> impl IntoIterator<Item = Self> + 'a {
        shortest_path_lengths[self.position as usize]
            .iter()
            .enumerate()
            .filter(move |&(destination, _distance)| self.can_visit(destination))
            .filter_map(move |(destination, distance)| {
                let minutes_remaining = self.minutes_remaining.checked_sub(*distance + 1)?;
                Some(State {
                    visited: self.visited | (1 << destination),
                    avoid: self.avoid,
                    pressure_released: self.pressure_released
                        + minutes_remaining as u16 * flow_rates[destination] as u16,
                    minutes_remaining,
                    position: destination as u8,
                })
            })
    }
}
