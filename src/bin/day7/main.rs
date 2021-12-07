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

fn total_fuel(positions: &[i64], align: i64) -> i64 {
    positions
        .iter()
        .map(|pos| fuel_needed((pos - align).abs()))
        .sum()
}

/// We can't just round the average, because the derivative is
/// align - avg + sum(signum(align - pos)) / 2N
/// As opposed to
/// align - avg.
/// Since the last term can never be more than 0.5,
/// We only need to check the floor and ceiling of avg
fn part_b(data: &'static str) -> i64 {
    let positions = parse(data);
    let avg = positions.iter().sum::<i64>() as f64 / positions.len() as f64;
    let floor = avg.floor() as i64;
    let ceil = avg.ceil() as i64;
    total_fuel(&positions, floor).min(total_fuel(&positions, ceil))
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
