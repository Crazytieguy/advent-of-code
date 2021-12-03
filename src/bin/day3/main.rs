use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> Vec<Vec<bool>> {
    data.lines()
        .map(|line| line.chars().map(|c| c == '1').collect())
        .collect()
}

fn part_a(data: &'static str) -> usize {
    let data = parse(data);
    let row_length = data[0].len();
    let most_common: Vec<bool> = (0..row_length)
        .map(|i| {
            let counts = data.iter().counts_by(|row| row[i]);
            counts[&true] > counts[&false]
        })
        .collect();
    let least_common = most_common.iter().map(|v| !v).collect::<Vec<_>>();
    let gamma = binary_to_number(&most_common);
    let epsilon = binary_to_number(&least_common);
    gamma * epsilon
}

fn binary_to_number(b: &[bool]) -> usize {
    b.iter()
        .rev()
        .enumerate()
        .map(|(i, bit)| if *bit { 2_usize.pow(i as u32) } else { 0 })
        .sum()
}

#[allow(dead_code)]
fn part_b(data: &'static str) -> usize {
    let data = parse(data);
    let oxygen_generator_rating = iterative_filter(&data, true);
    let co2_scrubber_rating = iterative_filter(&data, false);
    oxygen_generator_rating * co2_scrubber_rating
}

fn iterative_filter(data: &[Vec<bool>], keep_most_common: bool) -> usize {
    let mut i = 0;
    let mut remaining: Vec<_> = data.iter().collect();
    loop {
        let counts = remaining.iter().counts_by(|row| row[i]);
        let most_common = keep_most_common == (counts[&true] >= counts[&false]);
        remaining = remaining
            .into_iter()
            .filter(|row| row[i] == most_common)
            .collect();
        if remaining.len() == 1 {
            break binary_to_number(remaining[0]);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 198);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 230);
    }
}
