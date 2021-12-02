use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(derive_new::new)]
struct Position {
    distance: i32,
    aim: i32,
    depth: i32,
}

impl Position {
    fn apply_command(mut self, c: Command) -> Self {
        match c {
            Command::Forward(n) => {
                self.distance += n;
                self.depth += self.aim * n;
            }
            Command::Down(n) => {
                self.aim += n;
            }
            Command::Up(n) => {
                self.aim -= n;
            }
        };
        self
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
    let final_position = data
        .lines()
        .map(Command::from)
        .fold((0, 0), |acc, command| {
            let position = command.simple_position();
            (acc.0 + position.0, acc.1 + position.1)
        });
    final_position.0 * final_position.1
}

fn part_b(data: &str) -> i32 {
    let final_position = data
        .lines()
        .map(Command::from)
        .fold(Position::new(0, 0, 0), |acc, c| acc.apply_command(c));
    final_position.distance * final_position.depth
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
