use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", a(DATA));
    // println!("part b: {}", b(DATA));
}

fn a(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn b(data: &str) -> usize {
    data.lines()
        .map(|line| line.parse::<u32>().unwrap())
        .tuple_windows()
        .filter(|(a, _, _, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(a(SAMPLE_DATA), 7);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(SAMPLE_DATA), 5);
    }
}
