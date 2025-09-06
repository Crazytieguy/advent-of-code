use std::{
    borrow::Cow,
    collections::{HashMap, VecDeque},
};

use advent_2023::Solution;
use anyhow::{anyhow, bail};
use itertools::Itertools;
use num::Integer;
use winnow::{
    ascii::alpha1,
    combinator::{alt, empty, separated},
    seq, Parser,
};

struct Day;

#[derive(Debug, Clone, Default)]
struct Graph {
    destinations: HashMap<&'static str, Vec<&'static str>>,
    modules: HashMap<&'static str, Module>,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop {
        on: bool,
    },
    Conjunction {
        memory: HashMap<&'static str, PulseStrength>,
    },
    Broadcaster,
}

#[derive(Debug, Clone, Copy)]
struct Pulse {
    source: &'static str,
    destination: &'static str,
    strength: PulseStrength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PulseStrength {
    Low,
    High,
}

impl Solution for Day {
    const INPUT: &'static str = include_str!("data.txt");
    const SAMPLE_INPUT: &'static str = include_str!("sample.txt");

    type Shared = Graph;
    type Answer = u64;

    const SAMPLE_ANSWER_A: Self::TestAnswer = 11687500;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 0;

    fn shared(input: &'static str) -> anyhow::Result<Self::Shared> {
        let mut graph = Graph::default();
        for parsed_line in input
            .lines()
            .map(|line| node.parse(line).map_err(anyhow::Error::msg))
        {
            let (module, name, destinations) = parsed_line?;
            graph.modules.insert(name, module);
            graph.destinations.insert(name, destinations);
        }
        for (source, destinations) in &graph.destinations {
            for &destination in destinations {
                if let Some(Module::Conjunction { memory }) = graph.modules.get_mut(destination) {
                    memory.insert(source, PulseStrength::Low);
                }
            }
        }
        Ok(graph)
    }

    fn part_a(graph: Cow<Self::Shared>) -> anyhow::Result<Self::Answer> {
        let Graph {
            mut modules,
            destinations,
        } = graph.into_owned();
        let mut low_count = 0;
        let mut high_count = 0;
        for _ in 0..1000 {
            let mut queue = new_queue();
            while let Some(pulse) = queue.pop_front() {
                match pulse.strength {
                    PulseStrength::Low => low_count += 1,
                    PulseStrength::High => high_count += 1,
                }
                send_pulses(&mut modules, pulse, &destinations, &mut queue)?;
            }
        }
        Ok(low_count * high_count)
    }

    fn part_b(
        Graph {
            mut modules,
            destinations,
        }: Self::Shared,
    ) -> anyhow::Result<Self::Answer> {
        let &conjunction_before_rx = destinations
            .iter()
            .find_map(|(source, destinations)| destinations.contains(&"rx").then_some(source))
            .ok_or_else(|| anyhow!("no module that sends to rx"))?;
        let Some(Module::Conjunction { memory }) = modules.get(conjunction_before_rx) else {
            bail!("module that sends to rx is not a conjunction")
        };
        let mut to_track: HashMap<&str, Option<u64>> = memory.keys().map(|&k| (k, None)).collect();
        for presses in 1..100_000 {
            let mut queue = new_queue();
            while let Some(pulse) = queue.pop_front() {
                if pulse.destination == conjunction_before_rx
                    && matches!(pulse.strength, PulseStrength::High)
                {
                    *to_track.get_mut(pulse.source).ok_or_else(|| {
                        anyhow!("Pulse to conjunction before rx that wasn't tracked: {pulse:?}")
                    })? = Some(presses);
                    if let Some(lcm) = to_track
                        .values()
                        .try_fold(1, |acc, &cur| Some(acc.lcm(&cur?)))
                    {
                        return Ok(lcm);
                    }
                }
                send_pulses(&mut modules, pulse, &destinations, &mut queue)?;
            }
        }
        Err(anyhow!("Couldn't find a solution within 100,000 presses"))
    }

    fn shared_test(input: &'static str) -> anyhow::Result<Self::SharedTest> {
        Self::shared(input)
    }

    fn part_a_test(shared: Self::SharedTest) -> anyhow::Result<Self::Answer> {
        Self::part_a(Cow::Owned(shared))
    }

    fn part_b_test(_: Self::SharedTest) -> anyhow::Result<Self::Answer> {
        Ok(0)
    }
}

fn new_queue() -> VecDeque<Pulse> {
    VecDeque::from([Pulse {
        source: "button",
        destination: "broadcaster",
        strength: PulseStrength::Low,
    }])
}

fn send_pulses(
    modules: &mut HashMap<&'static str, Module>,
    pulse: Pulse,
    destinations: &HashMap<&'static str, Vec<&'static str>>,
    queue: &mut VecDeque<Pulse>,
) -> anyhow::Result<()> {
    let Some(module) = modules.get_mut(pulse.destination) else {
        return Ok(());
    };
    let send = match (module, pulse.strength) {
        (Module::FlipFlop { .. }, PulseStrength::High) => None,
        (Module::FlipFlop { on }, PulseStrength::Low) => {
            *on = !*on;
            Some(if *on {
                PulseStrength::High
            } else {
                PulseStrength::Low
            })
        }
        (Module::Conjunction { memory }, strength) => {
            *memory.get_mut(pulse.source).ok_or_else(|| {
                anyhow!("no memory for {} in {}", pulse.source, pulse.destination)
            })? = strength;
            Some(
                if memory.values().all_equal_value() == Ok(&PulseStrength::High) {
                    PulseStrength::Low
                } else {
                    PulseStrength::High
                },
            )
        }
        (Module::Broadcaster, strength) => Some(strength),
    };
    if let Some(strength) = send {
        for &destination in destinations
            .get(pulse.destination)
            .ok_or_else(|| anyhow!("no destinations for {}", pulse.destination))?
        {
            queue.push_back(Pulse {
                source: pulse.destination,
                destination,
                strength,
            });
        }
    };
    Ok(())
}

fn node(input: &mut &'static str) -> winnow::Result<(Module, &'static str, Vec<&'static str>)> {
    seq! {(
        alt((
            '%'.value(Module::FlipFlop { on: false }),
            '&'.value(Module::Conjunction {
                memory: HashMap::new(),
            }),
            empty.value(Module::Broadcaster)
        )),
        alpha1,
        _: " -> ",
        separated(1.., alpha1, ", ")
    )}
    .parse_next(input)
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
