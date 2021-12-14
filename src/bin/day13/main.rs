#![feature(destructuring_assignment)]
use itertools::{repeat_n, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("{}", part_b(DATA));
}

enum Fold {
    X(i64),
    Y(i64),
}

type Point = (i64, i64);

fn parse(data: &'static str) -> (Vec<Point>, Vec<Fold>) {
    let points = data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect_vec();
    let folds = data
        .lines()
        .skip_while(|line| !line.starts_with('f'))
        .map(|line| {
            let (axis, value) = line.split_once('=').unwrap();
            let value = value.parse().unwrap();
            match axis {
                "fold along x" => Fold::X(value),
                "fold along y" => Fold::Y(value),
                _ => panic!(),
            }
        })
        .collect_vec();
    (points, folds)
}

fn part_a(data: &'static str) -> usize {
    let (points, folds) = parse(data);
    apply_folds(points, &folds[0..=0]).len()
}

fn apply_folds(points: Vec<Point>, folds: &[Fold]) -> Vec<Point> {
    points
        .into_iter()
        .map(|p| {
            folds.iter().fold(p, |(x, y), fold| match *fold {
                Fold::X(v) => (v - (v - x).abs(), y),
                Fold::Y(v) => (x, v - (v - y).abs()),
            })
        })
        .unique()
        .collect()
}

#[allow(dead_code)]
fn part_b(data: &'static str) -> String {
    let (mut points, folds) = parse(data);
    points = apply_folds(points, &folds);
    points.sort_unstable_by(|(xa, ya), (xb, yb)| (ya, xa).cmp(&(yb, xb)));
    let mut picture = String::new();
    let (mut x, mut y) = (-1, 0);
    for (next_x, next_y) in points {
        if next_y != y {
            picture.extend(repeat_n('\n', (next_y - y) as usize));
            x = -1;
        }
        picture.extend(repeat_n(' ', (next_x - x - 1) as usize));
        picture.push('#');
        (x, y) = (next_x, next_y);
    }
    picture
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 17);
    }

    #[test]
    fn test_b() {
        assert_eq!(
            part_b(SAMPLE_DATA),
            "#####\n\
             #   #\n\
             #   #\n\
             #   #\n\
             #####"
        );
    }
}
