use advent_2022::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    multi::separated_list1,
    Parser,
};
use nom_supreme::ParserExt;
use std::{collections::HashMap, ops::Range};

boilerplate!(Day);

impl Solution for Day {
    type Parsed = Vec<Pair>;
    type Answer = i64;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 26;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 56_000_011;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        separated_list1(line_ending, pair)(data)
    }

    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest> {
        Self::parse(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        solve_a::<2_000_000>(data) as i64
    }

    fn a_test(data: Self::Parsed) -> Self::Answer {
        solve_a::<10>(data) as i64
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        solve_b::<4_000_000>(data)
    }

    fn b_test(data: Self::Parsed) -> Self::Answer {
        solve_b::<20>(data)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    sensor: Point,
    beacon: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

fn pair(input: &str) -> IResult<Pair> {
    let (input, x) = tag("Sensor at x=").precedes(i32).parse(input)?;
    let (input, y) = tag(", y=").precedes(i32).parse(input)?;
    let sensor = Point { x, y };
    let (input, x) = tag(": closest beacon is at x=")
        .precedes(i32)
        .parse(input)?;
    let (input, y) = tag(", y=").precedes(i32).parse(input)?;
    let beacon = Point { x, y };
    Ok((input, Pair { sensor, beacon }))
}

fn solve_a<const EXAMPLE_ROW: i32>(pairs: Vec<Pair>) -> usize {
    let count_covered_xs: usize = pairs
        .iter()
        .flat_map(|pair| pair.covered_xs(EXAMPLE_ROW))
        .sorted_unstable_by_key(|range| range.start)
        .coalesce(|a, b| {
            if a.end >= b.start {
                Ok(a.start..b.end.max(a.end))
            } else {
                Err((a, b))
            }
        })
        .map(|xs| xs.len())
        .sum();
    let blocked_xs = pairs
        .into_iter()
        .flat_map(|pair| [pair.sensor, pair.beacon])
        .filter(|p| p.y == EXAMPLE_ROW)
        .unique()
        .count();
    count_covered_xs - blocked_xs
}

fn solve_b<const MAX_COORD: i32>(pairs: Vec<Pair>) -> i64 {
    let top_right = pairs
        .iter()
        .map(Pair::top_right)
        .into_group_map_by(Line::y_intercept);
    let positive_slope_overlaps = pairs
        .iter()
        .map(Pair::bottom_left)
        .into_grouping_map_by(Line::y_intercept)
        .fold(vec![], fold_compatible_overlaps(&top_right));
    let top_left = pairs
        .iter()
        .map(Pair::top_left)
        .into_group_map_by(Line::y_intercept);
    let negative_slope_overlaps = pairs
        .iter()
        .map(Pair::bottom_right)
        .into_grouping_map_by(Line::y_intercept)
        .fold(vec![], fold_compatible_overlaps(&top_left));
    let Point { x, y } = positive_slope_overlaps
        .values()
        .flatten()
        .cartesian_product(negative_slope_overlaps.values().flatten())
        .find_map(|(positive, negative)| {
            positive.interception(negative).filter(|p| {
                p.x >= 0
                    && p.x <= MAX_COORD
                    && p.y >= 0
                    && p.y <= MAX_COORD
                    && pairs.iter().all(|pair| !pair.covers(p))
            })
        })
        .expect("should be an interception");

    x as i64 * 4_000_000 + y as i64
}

fn fold_compatible_overlaps(
    parallel: &HashMap<i32, Vec<Line>>,
) -> impl Fn(Vec<Line>, &i32, Line) -> Vec<Line> + '_ {
    |mut overlaps, y_intercept, line| {
        overlaps.extend(
            parallel
                .get(y_intercept)
                .iter()
                .flat_map(|v| v.iter())
                .filter_map(|other| line.overlap(other)),
        );
        overlaps
    }
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Pair {
    fn cover_distance(&self) -> i32 {
        self.sensor.manhattan_distance(&self.beacon)
    }

    fn top_left(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x - self.cover_distance() - 1,
                y: self.sensor.y,
            },
            end: Point {
                x: self.sensor.x,
                y: self.sensor.y - self.cover_distance() - 1,
            },
        }
    }

    fn top_right(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x,
                y: self.sensor.y - self.cover_distance() - 1,
            },
            end: Point {
                x: self.sensor.x + self.cover_distance() + 1,
                y: self.sensor.y,
            },
        }
    }

    fn bottom_left(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x - self.cover_distance() - 1,
                y: self.sensor.y,
            },
            end: Point {
                x: self.sensor.x,
                y: self.sensor.y + self.cover_distance() + 1,
            },
        }
    }

    fn bottom_right(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x,
                y: self.sensor.y + self.cover_distance() + 1,
            },
            end: Point {
                x: self.sensor.x + self.cover_distance() + 1,
                y: self.sensor.y,
            },
        }
    }

    fn covers(&self, point: &Point) -> bool {
        self.sensor.manhattan_distance(point) <= self.cover_distance()
    }

    fn covered_xs(&self, row: i32) -> Option<Range<i32>> {
        let x_offset = self.cover_distance() - (self.sensor.y - row).abs();
        Some(self.sensor.x - x_offset..self.sensor.x + x_offset + 1).filter(|r| !r.is_empty())
    }
}

impl Line {
    fn slope(&self) -> i32 {
        if self.end.y - self.start.y > 0 {
            1
        } else {
            -1
        }
    }

    fn y_intercept(&self) -> i32 {
        self.start.y - self.slope() * self.start.x
    }

    fn y_at(&self, x: i32) -> Option<i32> {
        if self.start.x > x || self.end.x < x {
            return None;
        }
        Some(self.slope() * x + self.y_intercept())
    }

    fn overlap(&self, other: &Line) -> Option<Line> {
        debug_assert_eq!(self.slope(), other.slope());
        debug_assert_eq!(self.y_intercept(), other.y_intercept());
        let x = self.start.x.max(other.start.x);
        let y = self.y_at(x)?;
        let start = Point { x, y };

        let x = self.end.x.min(other.end.x);
        let y = self.y_at(x)?;
        let end = Point { x, y };
        Some(Line { start, end })
    }

    fn interception(&self, other: &Line) -> Option<Point> {
        debug_assert_ne!(self.slope(), other.slope());
        let y_intercept_diff = other.y_intercept() - self.y_intercept();
        let slope_diff = self.slope() - other.slope();
        let x = y_intercept_diff / slope_diff;
        let y = self.y_at(x)?;
        if y != other.y_at(x)? {
            return None;
        }
        Some(Point { x, y })
    }
}
