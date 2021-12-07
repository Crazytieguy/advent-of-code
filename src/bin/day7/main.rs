use itertools::{Itertools, MinMaxResult};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> Vec<i64> {
    data.trim().split(',').map(|n| n.parse().unwrap()).collect()
}

fn part_a(data: &'static str) -> i64 {
    let mut positions = parse(data);
    positions.sort_unstable();
    let align = positions[positions.len() / 2];
    positions.iter().map(|pos| (pos - align).abs()).sum()
}

fn fuel_needed(steps: i64) -> i64 {
    (steps + 1) * steps / 2
}

fn part_b(data: &'static str) -> i64 {
    let positions = parse(data);
    if let MinMaxResult::MinMax(&min, &max) = positions.iter().minmax() {
        (min..max)
            .map(|align| {
                positions
                    .iter()
                    .map(|pos| fuel_needed((pos - align).abs()))
                    .sum::<i64>()
            })
            .min()
            .unwrap()
    } else {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 37);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 168);
    }
}
