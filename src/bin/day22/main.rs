use derive_new::new;
use itertools::Itertools;
use ndarray::{s, Array3};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

type Bounds = (i32, i32);

#[derive(Debug, Clone, Copy, new)]
struct Cuboid {
    x: Bounds,
    y: Bounds,
    z: Bounds,
}

impl Cuboid {
    fn size(&self) -> i64 {
        0.max(self.x.1 - self.x.0) as i64
            * 0.max(self.y.1 - self.y.0) as i64
            * 0.max(self.z.1 - self.z.0) as i64
    }

    fn is_valid(&self) -> bool {
        self.size() > 0
    }

    fn intersection(&self, other: Cuboid) -> Option<Cuboid> {
        let c = Cuboid {
            x: (self.x.0.max(other.x.0), self.x.1.min(other.x.1)),
            y: (self.y.0.max(other.y.0), self.y.1.min(other.y.1)),
            z: (self.z.0.max(other.z.0), self.z.1.min(other.z.1)),
        };
        if c.is_valid() {
            Some(c)
        } else {
            None
        }
    }

    fn is_init(&self) -> bool {
        self.x.0 >= -50
            && self.x.1 <= 51
            && self.y.0 >= -50
            && self.y.1 <= 51
            && self.z.0 >= -50
            && self.z.1 <= 51
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    on: bool,
    cuboid: Cuboid,
}

impl Step {
    fn volume(&self) -> i64 {
        if self.on {
            self.cuboid.size()
        } else {
            -self.cuboid.size()
        }
    }
}

fn parse(data: &'static str) -> Vec<Step> {
    data.lines()
        .map(|line| {
            let (command_str, rest) = line.split_once(' ').unwrap();
            let (x, y, z) = rest
                .split(',')
                .map(|part| {
                    let (min, max) = &part[2..].split_once("..").unwrap();
                    (min.parse().unwrap(), max.parse::<i32>().unwrap() + 1)
                })
                .collect_tuple()
                .unwrap();
            Step {
                on: command_str == "on",
                cuboid: Cuboid { x, y, z },
            }
        })
        .collect()
}

fn do_step(mut volumes: Vec<Step>, step: Step) -> Vec<Step> {
    volumes.extend(
        volumes
            .iter()
            .flat_map(|s| {
                s.cuboid.intersection(step.cuboid).map(|c| Step {
                    on: !s.on,
                    cuboid: c,
                })
            })
            .collect_vec(),
    );
    if step.on {
        volumes.push(step)
    }
    volumes
}

fn init_cubes_on(steps: Vec<Step>) -> i64 {
    let mut cubes = Array3::from_elem([101, 101, 101], false);
    for step in steps {
        let mut slice = cubes.slice_mut(s![
            step.cuboid.x.0 + 50..step.cuboid.x.1 + 50,
            step.cuboid.y.0 + 50..step.cuboid.y.1 + 50,
            step.cuboid.z.0 + 50..step.cuboid.z.1 + 50,
        ]);
        slice.fill(step.on);
    }
    cubes.fold(0, |acc, &cur| acc + cur as i64)
}

fn part_a(data: &'static str) -> i64 {
    let steps = parse(data);
    init_cubes_on(steps.into_iter().filter(|s| s.cuboid.is_init()).collect())
}

fn part_b(data: &'static str) -> i64 {
    let steps = parse(data);
    let (steps_init, steps_rest): (Vec<_>, Vec<_>) =
        steps.into_iter().partition(|s| s.cuboid.is_init());
    let rest_volumes = steps_rest.into_iter().fold(Vec::new(), do_step);
    init_cubes_on(steps_init) + rest_volumes.into_iter().map(|s| s.volume()).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_A: &str = include_str!("sample_a.txt");
    const SAMPLE_B: &str = include_str!("sample_b.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_A), 590784);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_B), 2758514936282235);
    }
}
