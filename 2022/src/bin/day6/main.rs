use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn solve<const N: usize>(data: &str) -> usize {
    data.as_bytes()
        .windows(N)
        .position(|window| window.iter().all_unique())
        .expect("There should be an all unique sequence")
        + N
}

fn part_a(data: &str) -> usize {
    solve::<4>(data)
}

fn part_b(data: &str) -> usize {
    solve::<14>(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 7);
        println!("part a: {}", part_a(DATA));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 19);
        println!("part b: {}", part_b(DATA));
    }
}

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}
