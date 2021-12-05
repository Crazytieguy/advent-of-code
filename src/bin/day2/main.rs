use itertools::Itertools;
use Command::{Down, Forward, Up};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let (direction, amount) = s.split_whitespace().collect_tuple().unwrap();
        let n = amount.parse().unwrap();
        match direction {
            "forward" => Forward(n),
            "down" => Down(n),
            "up" => Up(n),
            _ => panic!(),
        }
    }
}

fn part_a(data: &str) -> i32 {
    let mut dist = 0;
    let mut depth = 0;
    for command in data.lines().map(Command::from) {
        match command {
            Forward(n) => dist += n,
            Down(n) => depth += n,
            Up(n) => depth -= n,
        }
    }
    dist * depth
}

fn part_b(data: &str) -> i32 {
    let mut dist = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in data.lines().map(Command::from) {
        match command {
            Forward(n) => {
                dist += n;
                depth += aim * n
            }
            Down(n) => aim += n,
            Up(n) => aim -= n,
        }
    }
    dist * depth
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 150);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 900);
    }
}
