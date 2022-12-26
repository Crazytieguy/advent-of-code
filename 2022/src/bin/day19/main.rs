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

#[derive(Debug, Clone, Copy, Default)]
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
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(rhs.ore)?,
            clay: self.clay.checked_sub(rhs.clay)?,
            obsidian: self.obsidian.checked_sub(rhs.obsidian)?,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
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
        .map(|ore| ONE_ORE * ore)
        .parse(input)?;
    let (input, clay_robot_cost) = delimited(tag("Each clay robot costs "), u8, tag(" ore. "))
        .map(|ore| ONE_ORE * ore)
        .parse(input)?;
    let (input, obsidian_robot_cost) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(u8, tag(" ore and "), u8),
        tag(" clay. "),
    )
    .map(|(ore, clay)| ONE_ORE * ore + ONE_CLAY * clay)
    .parse(input)?;
    let (input, geode_robot_cost) = delimited(
        tag("Each geode robot costs "),
        separated_pair(u8, tag(" ore and "), u8),
        tag(" obsidian."),
    )
    .map(|(ore, obsidian)| ONE_ORE * ore + ONE_OBSIDIAN * obsidian)
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

    fn chose_robot(self, cost: Resources, robot: Resources) -> Option<Self> {
        (1..self.minutes_remaining).rev().zip(0..).find_map(
            |(minutes_remaining, minutes_passed)| {
                let resources = self.resources + self.resources_rate * minutes_passed;
                resources.checked_sub(cost).map(|resources| Self {
                    minutes_remaining,
                    resources: resources + self.resources_rate,
                    resources_rate: self.resources_rate + robot,
                    ..self
                })
            },
        )
    }

    fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> + '_ {
        let max_ore_cost = blueprint
            .clay_robot_cost
            .ore
            .max(blueprint.obsidian_robot_cost.ore)
            .max(blueprint.geode_robot_cost.ore);
        let ore_robot_viable = self.resources_rate.ore < max_ore_cost;
        let clay_robot_viable = self.resources_rate.clay < blueprint.obsidian_robot_cost.clay;
        let obsidian_robot_viable = self.resources_rate.obsidian
            < blueprint.geode_robot_cost.obsidian
            && self.resources_rate.clay > 0;
        let geode_robot_viable = self.resources_rate.obsidian > 0;
        [
            ore_robot_viable.then(|| self.chose_robot(blueprint.ore_robot_cost, ONE_ORE)),
            clay_robot_viable.then(|| self.chose_robot(blueprint.clay_robot_cost, ONE_CLAY)),
            obsidian_robot_viable
                .then(|| self.chose_robot(blueprint.obsidian_robot_cost, ONE_OBSIDIAN)),
            geode_robot_viable.then(|| {
                self.chose_robot(blueprint.geode_robot_cost, Default::default())
                    .map(|state| Self {
                        geodes_secured: state.geodes_secured + state.minutes_remaining,
                        ..state
                    })
            }),
        ]
        .into_iter()
        .flatten()
        .flatten()
    }

    // we have unlimited ore and clay, and prefer building geode robots when possible
    fn bound(self, blueprint: &Blueprint) -> u8 {
        let geode_cost = blueprint.geode_robot_cost.obsidian;
        let (_, _, geodes) = (0..self.minutes_remaining).rev().fold(
            (
                self.resources.obsidian,
                self.resources_rate.obsidian,
                self.geodes_secured,
            ),
            |(obsidian, rate, geodes), minutes_remaining| {
                if obsidian >= geode_cost {
                    (
                        obsidian + rate - geode_cost,
                        rate,
                        geodes.saturating_add(minutes_remaining),
                    )
                } else {
                    (obsidian + rate, rate + 1, geodes)
                }
            },
        );
        geodes
    }
}

fn branch_and_bound(blueprint: &Blueprint, state: State, best: &mut u8) {
    *best = state.geodes_secured.max(*best);
    for state in state.branch(blueprint) {
        if state.bound(blueprint) > *best {
            branch_and_bound(blueprint, state, best);
        }
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
