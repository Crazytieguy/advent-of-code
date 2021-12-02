const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    // println!("part b: {}", b(DATA));
}

struct InputRecord(usize);

impl From<&str> for InputRecord {
    fn from(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

fn part_a(data: &str) -> usize {
    data.lines()
        .map(InputRecord::from)
        .fold(0, |acc, line| acc + line.0)
}

#[allow(dead_code)]
fn part_b(data: &str) -> usize {
    data.lines()
        .map(InputRecord::from)
        .fold(0, |acc, line| acc + line.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 0);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 0);
    }
}
