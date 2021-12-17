use itertools::{iterate, Itertools};
use regex::Regex;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> (isize, isize, isize, isize) {
    let pat = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
    let captures = pat.captures(data).unwrap();
    (1..=4)
        .map(|i| captures[i].parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part_a(data: &'static str) -> usize {
    let (_min_x, _max_x, min_y, _max_y) = parse(data);
    let vel_y = -1 - min_y;
    (vel_y * (1 + vel_y) / 2) as usize
}

fn step(&(x, y, vel_x, vel_y): &(isize, isize, isize, isize)) -> (isize, isize, isize, isize) {
    let x = x + vel_x;
    let y = y + vel_y;
    let vel_x = vel_x - vel_x.signum();
    let vel_y = vel_y - 1;
    (x, y, vel_x, vel_y)
}

fn part_b(data: &'static str) -> usize {
    let (min_x, max_x, min_y, max_y) = parse(data);
    let mut init_count = 0;
    for (vel_x, vel_y) in (1..=max_x).cartesian_product(min_y..-min_y) {
        for (x, y, _vel_x, _vel_y) in iterate((0, 0, vel_x, vel_y), step) {
            if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                init_count += 1;
                break;
            }
            if x > max_x || y < min_y {
                break;
            }
        }
    }
    init_count
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 45);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 112);
    }
}
