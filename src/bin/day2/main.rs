use std::str::FromStr;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", a(DATA));
    println!("part b: {}", b(DATA));
}

struct Position {
    distance: i32,
    aim: i32,
    depth: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            distance: 0,
            aim: 0,
            depth: 0,
        }
    }

    fn apply_command(self, c: Command) -> Self {
        match c {
            Command::Forward(n) => Position {
                distance: self.distance + n,
                depth: self.depth + self.aim * n,
                ..self
            },
            Command::Down(n) => Position {
                aim: self.aim + n,
                ..self
            },
            Command::Up(n) => Position {
                aim: self.aim - n,
                ..self
            },
        }
    }
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn simple_position(&self) -> (i32, i32) {
        match self {
            Self::Forward(n) => (*n, 0),
            Self::Down(n) => (0, *n),
            Self::Up(n) => (0, -n),
        }
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let (direction, distance) = (
            tokens.next().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
        );
        Ok(if direction == "forward" {
            Self::Forward(distance)
        } else if direction == "down" {
            Self::Down(distance)
        } else {
            Self::Up(distance)
        })
    }
}

fn a(data: &str) -> i32 {
    let final_position = data
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .fold((0, 0), |acc, command| {
            let position = command.simple_position();
            (acc.0 + position.0, acc.1 + position.1)
        });
    final_position.0 * final_position.1
}

fn b(data: &str) -> i32 {
    let final_position = data
        .lines()
        .map(|line| line.parse::<Command>().unwrap())
        .fold(Position::new(), |acc, c| acc.apply_command(c));
    final_position.distance * final_position.depth
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(a(SAMPLE_DATA), 150);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(SAMPLE_DATA), 900);
    }
}
