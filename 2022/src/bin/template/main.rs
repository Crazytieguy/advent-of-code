const DATA: &str = include_str!("data.txt");

fn parse(data: &'static str) {
    println!("{}", data)
}

fn part_a(data: &'static str) -> usize {
    parse(data);
    0
}

fn part_b(data: &'static str) -> usize {
    parse(data);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 0);
        println!("part a: {}", part_a(DATA));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 0);
        println!("part b: {}", part_b(DATA));
    }
}

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}
