use derive_new::new;
use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

type Bounds = (i64, i64);

#[derive(Debug, Clone, Copy, new)]
struct Cuboid {
    x: Bounds,
    y: Bounds,
    z: Bounds,
}

impl Cuboid {
    fn size(&self) -> usize {
        (0.max(self.x.1 - self.x.0) * 0.max(self.y.1 - self.y.0) * 0.max(self.z.1 - self.z.0))
            as usize
    }

    fn is_valid(&self) -> bool {
        self.size() > 0
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    on: bool,
    cuboid: Cuboid,
}

fn parse(data: &'static str) -> Vec<Step> {
    data.lines()
        .map(|line| {
            let (command_str, rest) = line.split_once(' ').unwrap();
            let (x, y, z) = rest
                .split(',')
                .map(|part| {
                    let (min, max) = &part[2..].split_once("..").unwrap();
                    (min.parse().unwrap(), max.parse::<i64>().unwrap() + 1)
                })
                .collect_tuple()
                .unwrap();
            let on = match command_str {
                "on" => true,
                "off" => false,
                _ => panic!(),
            };
            Step {
                on,
                cuboid: Cuboid { x, y, z },
            }
        })
        .collect()
}

fn subtract_cuboids(lhs: Cuboid, rhs: Cuboid) -> [Cuboid; 6] {
    [
        Cuboid::new((lhs.x.0, lhs.x.1.min(rhs.x.0)), lhs.y, lhs.z),
        Cuboid::new(
            (rhs.x.0.max(lhs.x.0), rhs.x.1.min(lhs.x.1)),
            (lhs.y.0, lhs.y.1.min(rhs.y.0)),
            lhs.z,
        ),
        Cuboid::new(
            (rhs.x.0.max(lhs.x.0), rhs.x.1.min(lhs.x.1)),
            (rhs.y.0.max(lhs.y.0), rhs.y.1.min(lhs.y.1)),
            (lhs.z.0, lhs.z.1.min(rhs.z.0)),
        ),
        Cuboid::new(
            (rhs.x.0.max(lhs.x.0), rhs.x.1.min(lhs.x.1)),
            (rhs.y.0.max(lhs.y.0), rhs.y.1.min(lhs.y.1)),
            (rhs.z.1.max(lhs.z.0), lhs.z.1),
        ),
        Cuboid::new(
            (rhs.x.0.max(lhs.x.0), rhs.x.1.min(lhs.x.1)),
            (rhs.y.1.max(lhs.y.0), lhs.y.1),
            lhs.z,
        ),
        Cuboid::new((rhs.x.1.max(lhs.x.0), lhs.x.1), lhs.y, lhs.z),
    ]
}

fn do_step(non_contiguous_cuboids: Vec<Cuboid>, step: Step) -> Vec<Cuboid> {
    non_contiguous_cuboids
        .into_iter()
        .flat_map(|lhs| subtract_cuboids(lhs, step.cuboid))
        .filter(Cuboid::is_valid)
        .chain(if step.on { Some(step.cuboid) } else { None })
        .collect()
}

fn part_a(data: &'static str) -> usize {
    let steps = parse(data);
    let non_contiguous_cuboids = steps
        .into_iter()
        .filter(|s| {
            s.cuboid.x.0 >= -50
                && s.cuboid.x.1 <= 51
                && s.cuboid.y.0 >= -50
                && s.cuboid.y.1 <= 51
                && s.cuboid.z.0 >= -50
                && s.cuboid.z.1 <= 51
        })
        .fold(Vec::new(), do_step);
    non_contiguous_cuboids.into_iter().map(|c| c.size()).sum()
}

fn part_b(data: &'static str) -> usize {
    let steps = parse(data);
    let non_contiguous_cuboids = steps.into_iter().fold(Vec::new(), do_step);
    non_contiguous_cuboids.into_iter().map(|c| c.size()).sum()
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
