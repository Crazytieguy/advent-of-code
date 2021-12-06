use std::iter;

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

struct IncrDecrInclusiveRange {
    state: i32,
    stop: i32,
    signum: i32,
    done: bool,
}

impl IncrDecrInclusiveRange {
    fn new(start: i32, stop: i32) -> Self {
        Self {
            state: start,
            stop,
            signum: (stop - start).signum(),
            done: false,
        }
    }
}

impl Iterator for IncrDecrInclusiveRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state == self.stop {
            if self.done {
                None
            } else {
                self.done = true;
                Some(self.state)
            }
        } else {
            let res = self.state;
            self.state += self.signum;
            Some(res)
        }
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
        Self { x1, y1, x2, y2 }
    }
}

fn part_a(data: &'static str) -> usize {
    data.lines()
        .map(Vent::from)
        .flat_map(|Vent { x1, x2, y1, y2 }| {
            if x1 == x2 {
                iter::repeat(x1)
                    .zip(IncrDecrInclusiveRange::new(y1, y2))
                    .collect()
            } else if y1 == y2 {
                IncrDecrInclusiveRange::new(x1, x2)
                    .zip(iter::repeat(y1))
                    .collect()
            } else {
                vec![]
            }
        })
        .counts()
        .values()
        .filter(|&v| *v >= 2)
        .count()
}

#[allow(dead_code)]
fn part_b(data: &'static str) -> usize {
    data.lines()
        .map(Vent::from)
        .flat_map(|Vent { x1, x2, y1, y2 }| {
            if x1 == x2 {
                iter::repeat(x1)
                    .zip(IncrDecrInclusiveRange::new(y1, y2))
                    .collect_vec()
            } else if y1 == y2 {
                IncrDecrInclusiveRange::new(x1, x2)
                    .zip(iter::repeat(y1))
                    .collect_vec()
            } else {
                IncrDecrInclusiveRange::new(x1, x2)
                    .zip(IncrDecrInclusiveRange::new(y1, y2))
                    .collect_vec()
            }
        })
        .counts()
        .values()
        .filter(|&v| *v >= 2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_range() {
        let mut range = IncrDecrInclusiveRange::new(0, 2);
        assert_eq!(range.next(), Some(0));
        assert_eq!(range.next(), Some(1));
        assert_eq!(range.next(), Some(2));
        assert_eq!(range.next(), None);

        range = IncrDecrInclusiveRange::new(2, 0);
        assert_eq!(range.next(), Some(2));
        assert_eq!(range.next(), Some(1));
        assert_eq!(range.next(), Some(0));
        assert_eq!(range.next(), None);
    }

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 5);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 12);
    }
}
