use std::collections::HashMap;

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

fn grid_compression(steps: &[Step]) -> i64 {
    let (mut xs, (mut ys, mut zs)): (Vec<_>, (Vec<_>, Vec<_>)) = steps
        .iter()
        .flat_map(|s| {
            [
                (s.cuboid.x.0, (s.cuboid.y.0, s.cuboid.z.0)),
                (s.cuboid.x.1, (s.cuboid.y.1, s.cuboid.z.1)),
            ]
        })
        .unzip();
    xs.sort_unstable();
    ys.sort_unstable();
    zs.sort_unstable();
    let xs_map: HashMap<_, _> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let ys_map: HashMap<_, _> = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();
    let zs_map: HashMap<_, _> = zs.iter().enumerate().map(|(i, &z)| (z, i)).collect();
    let mut cubes_compressed = Array3::from_elem([xs.len() - 1, ys.len() - 1, zs.len() - 1], false);
    for step in steps {
        let x0 = xs_map[&step.cuboid.x.0];
        let x1 = xs_map[&step.cuboid.x.1];
        let y0 = ys_map[&step.cuboid.y.0];
        let y1 = ys_map[&step.cuboid.y.1];
        let z0 = zs_map[&step.cuboid.z.0];
        let z1 = zs_map[&step.cuboid.z.1];
        let mut slice = cubes_compressed.slice_mut(s![x0..x1, y0..y1, z0..z1]);
        slice.fill(step.on);
    }
    cubes_compressed
        .indexed_iter()
        .map(|(i, &on)| {
            on as i64
                * (xs[i.0 + 1] - xs[i.0]) as i64
                * (ys[i.1 + 1] - ys[i.1]) as i64
                * (zs[i.2 + 1] - zs[i.2]) as i64
        })
        .sum()
}

fn part_a_calc(steps: &[Step]) -> i64 {
    grid_compression(
        &steps
            .iter()
            .take_while(|s| s.cuboid.is_init())
            .copied()
            .collect_vec(),
    )
}

fn step_volumes(mut volumes: Vec<Step>, step: Step) -> Vec<Step> {
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

fn part_b_calc(steps: Vec<Step>) -> i64 {
    let volumes = steps
        .into_iter()
        .skip_while(|s| s.cuboid.is_init())
        .fold(Vec::new(), step_volumes);
    volumes.into_iter().map(|s| s.volume()).sum()
}

fn part_a(data: &'static str) -> i64 {
    part_a_calc(&parse(data))
}

fn part_b(data: &'static str) -> i64 {
    let steps = parse(data);
    part_a_calc(&steps) + part_b_calc(steps)
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
