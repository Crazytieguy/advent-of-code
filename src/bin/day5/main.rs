use bevy_math::{ivec2, IVec2};
use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

struct Vent {
    start: IVec2,
    stop: IVec2,
}

impl Vent {
    fn points(&self) -> Vec<IVec2> {
        let mut points = vec![];
        let mut cur = self.start;
        let direction = (self.stop - self.start).signum();
        while cur != self.stop {
            points.push(cur);
            cur += direction;
        }
        points.push(cur);
        points
    }
}

impl From<&'static str> for Vent {
    fn from(s: &'static str) -> Self {
        let (x1, y1, x2, y2) = s
            .split(" -> ")
            .flat_map(|point| point.split(','))
            .map(|coord| coord.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self {
            start: ivec2(x1, y1),
            stop: ivec2(x2, y2),
        }
    }
}

fn part_a(data: &'static str) -> usize {
    data.lines()
        .map(Vent::from)
        .flat_map(|vent| {
            if vent.start.x == vent.stop.x || vent.start.y == vent.stop.y {
                vent.points()
            } else {
                vec![]
            }
        })
        .counts()
        .values()
        .filter(|&&v| v >= 2)
        .count()
}

#[allow(dead_code)]
fn part_b(data: &'static str) -> usize {
    data.lines()
        .map(Vent::from)
        .flat_map(|vent| vent.points())
        .counts()
        .values()
        .filter(|&&v| v >= 2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 5);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 12);
    }
}
