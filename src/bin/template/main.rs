use std::str::FromStr;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", a(DATA));
    // println!("part b: {}", b(DATA));
}

struct InputLine(usize);

impl FromStr for InputLine {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().unwrap()))
    }
}

fn a(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<InputLine>().unwrap())
        .fold(0, |acc, line| acc + line.0)
}

#[allow(dead_code)]
fn b(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<InputLine>().unwrap())
        .fold(0, |acc, line| acc + line.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(a(SAMPLE_DATA), 0);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(SAMPLE_DATA), 0);
    }
}
