const DATA: &str = include_str!("data.txt");

type Parsed = &'static str;

fn parse(data: &'static str) -> Parsed {
    data
}

fn part_a(data: &Parsed) -> usize {
    println!("{data:?}");
    0
}

fn part_b(data: &Parsed) -> usize {
    println!("{data:?}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 0);
        println!("part a: {}", part_a(&parse(DATA)));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 0);
        println!("part b: {}", part_b(&parse(DATA)));
    }
}

fn main() {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
}
