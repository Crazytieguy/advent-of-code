use itertools::Itertools;

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
        let (command, n) = s.split_whitespace().collect_tuple().unwrap();
        let n = n.parse().unwrap();
        match command {
            "forward" => Self::Forward(n),
            "down" => Self::Down(n),
            "up" => Self::Up(n),
            _ => panic!(),
        }
    }
}

fn part_a(data: &str) -> i32 {
    let mut dist = 0;
    let mut depth = 0;
    for command in data.lines().map(Command::from) {
        match command {
            Command::Forward(n) => dist += n,
            Command::Down(n) => depth += n,
            Command::Up(n) => depth -= n,
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
            Command::Forward(n) => {
                dist += n;
                depth += aim * n
            }
            Command::Down(n) => aim += n,
            Command::Up(n) => aim -= n,
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
