use std::{
    error::Error,
    ops::{Add, Mul},
};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u8},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Parser,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed = Vec<Blueprint>;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

const ONE_ORE: Resources = Resources {
    ore: 1,
    clay: 0,
    obsidian: 0,
};
const ONE_CLAY: Resources = Resources {
    ore: 0,
    clay: 1,
    obsidian: 0,
};
const ONE_OBSIDIAN: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 1,
};

impl Resources {
    fn checked_sub(self, other: Self) -> Option<Self> {
        let ore = self.ore.checked_sub(other.ore)?;
        let clay = self.clay.checked_sub(other.clay)?;
        let obsidian = self.obsidian.checked_sub(other.obsidian)?;
        Some(Self {
            ore,
            clay,
            obsidian,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
        }
    }
}

impl Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, other: u8) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u8,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

fn blueprint(input: &str) -> IResult<Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), u8, tag(": "))(input)?;
    let (input, ore_robot_cost) = delimited(tag("Each ore robot costs "), u8, tag(" ore. "))
        .map(|ore| Resources {
            ore,
            ..Default::default()
        })
        .parse(input)?;
    let (input, clay_robot_cost) = delimited(tag("Each clay robot costs "), u8, tag(" ore. "))
        .map(|ore| Resources {
            ore,
            ..Default::default()
        })
        .parse(input)?;
    let (input, obsidian_robot_cost) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(u8, tag(" ore and "), u8),
        tag(" clay. "),
    )
    .map(|(ore, clay)| Resources {
        ore,
        clay,
        ..Default::default()
    })
    .parse(input)?;
    let (input, geode_robot_cost) = delimited(
        tag("Each geode robot costs "),
        separated_pair(u8, tag(" ore and "), u8),
        tag(" obsidian."),
    )
    .map(|(ore, obsidian)| Resources {
        ore,
        obsidian,
        ..Default::default()
    })
    .parse(input)?;
    Ok((
        input,
        Blueprint {
            id,
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost,
            geode_robot_cost,
        },
    ))
}

fn parse(data: &str) -> IResult<Parsed> {
    separated_list1(line_ending, blueprint)(data)
}

#[derive(Debug, Clone, Copy)]
struct State {
    minutes_remaining: u8,
    geodes_secured: u8,
    resources: Resources,
    resources_rate: Resources,
}

impl State {
    fn new(minutes_remaining: u8) -> Self {
        Self {
            minutes_remaining,
            geodes_secured: 0,
            resources: Default::default(),
            resources_rate: ONE_ORE,
        }
    }

    fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> + '_ {
        [
            if self.resources_rate.obsidian == 0 {
                None
            } else {
                (1..self.minutes_remaining).rev().zip(0..).find_map(
                    |(minutes_remaining, minutes_passed)| {
                        let resources = self.resources + self.resources_rate * minutes_passed;
                        resources
                            .checked_sub(blueprint.geode_robot_cost)
                            .map(|resources| State {
                                minutes_remaining,
                                geodes_secured: self.geodes_secured + minutes_remaining,
                                resources: resources + self.resources_rate,
                                ..self
                            })
                    },
                )
            },
            if self.resources_rate.clay == 0
                || self.resources_rate.obsidian == blueprint.geode_robot_cost.obsidian
            {
                None
            } else {
                (1..self.minutes_remaining).rev().zip(0..).find_map(
                    |(minutes_remaining, minutes_passed)| {
                        let resources = self.resources + self.resources_rate * minutes_passed;
                        resources
                            .checked_sub(blueprint.obsidian_robot_cost)
                            .map(|resources| State {
                                minutes_remaining,
                                resources: resources + self.resources_rate,
                                resources_rate: self.resources_rate + ONE_OBSIDIAN,
                                ..self
                            })
                    },
                )
            },
            if self.resources_rate.clay == blueprint.obsidian_robot_cost.clay {
                None
            } else {
                (1..self.minutes_remaining).rev().zip(0..).find_map(
                    |(minutes_remaining, minutes_passed)| {
                        let resources = self.resources + self.resources_rate * minutes_passed;
                        resources
                            .checked_sub(blueprint.clay_robot_cost)
                            .map(|resources| State {
                                minutes_remaining,
                                resources: resources + self.resources_rate,
                                resources_rate: self.resources_rate + ONE_CLAY,
                                ..self
                            })
                    },
                )
            },
            if self.resources_rate.ore
                == blueprint
                    .ore_robot_cost
                    .ore
                    .max(blueprint.clay_robot_cost.ore)
                    .max(blueprint.obsidian_robot_cost.ore)
                    .max(blueprint.geode_robot_cost.ore)
            {
                None
            } else {
                (1..self.minutes_remaining).rev().zip(0..).find_map(
                    |(minutes_remaining, minutes_passed)| {
                        let resources = self.resources + self.resources_rate * minutes_passed;
                        resources
                            .checked_sub(blueprint.ore_robot_cost)
                            .map(|resources| State {
                                minutes_remaining,
                                resources: resources + self.resources_rate,
                                resources_rate: self.resources_rate + ONE_ORE,
                                ..self
                            })
                    },
                )
            },
        ]
        .into_iter()
        .flatten()
    }

    fn bound(self) -> u8 {
        (1..self.minutes_remaining)
            .chain([self.geodes_secured])
            .fold(0, |acc, cur| acc.saturating_add(cur))
    }
}

fn branch_and_bound(blueprint: &Blueprint, state: State, best: &mut u8) {
    *best = state.geodes_secured.max(*best);
    if state.bound() <= *best {
        return;
    }
    for state in state.branch(blueprint) {
        branch_and_bound(blueprint, state, best);
    }
}

fn part_a(data: &Parsed) -> usize {
    data.iter()
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(blueprint, State::new(24), &mut best);
            blueprint.id as usize * best as usize
        })
        .sum()
}

fn part_b(data: &Parsed) -> usize {
    data.iter()
        .take(3)
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(blueprint, State::new(32), &mut best);
            best as usize
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 33);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 56 * 62);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let start = std::time::Instant::now();
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    println!("time: {:?}", start.elapsed());
    Ok(())
}
