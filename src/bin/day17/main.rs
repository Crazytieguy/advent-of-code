use itertools::{iterate, Itertools};
use regex::Regex;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> [isize; 4] {
    let pat = Regex::new(r"target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)").unwrap();
    let caps = pat.captures(data).unwrap();
    [&caps[1], &caps[2], &caps[3], &caps[4]].map(|s| s.parse().unwrap())
}

fn part_a(data: &'static str) -> usize {
    let [_min_x, _max_x, min_y, _max_y] = parse(data);
    let vel_y = -1 - min_y;
    (vel_y * (1 + vel_y) / 2) as usize
}

fn part_b(data: &'static str) -> usize {
    let [min_x, max_x, min_y, max_y] = parse(data);
    let possible_vels_x = 1..=max_x;
    let possible_vels_y = min_y..-min_y;
    (possible_vels_x)
        .cartesian_product(possible_vels_y)
        .filter(|&(vel_x, vel_y)| {
            iterate((0, 0, vel_x, vel_y), |&(x, y, vel_x, vel_y)| {
                (x + vel_x, y + vel_y, vel_x - vel_x.signum(), vel_y - 1)
            })
            .take_while(|&(x, y, _, _)| x <= max_x && y >= min_y)
            .any(|(x, y, _, _)| min_x <= x && x <= max_x && min_y <= y && y <= max_y)
        })
        .count()
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
