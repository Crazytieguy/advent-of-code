use std::fs;

use itertools::Itertools;

pub fn a() -> usize {
    _a(&get_input_data())
}

pub fn b() -> usize {
    _b(&get_input_data())
}

fn get_input_data() -> Vec<u32> {
    fs::read_to_string("data/day1")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn _a(data: &[u32]) -> usize {
    data.iter().tuple_windows().filter(|(a, b)| b > a).count()
}

fn _b(data: &[u32]) -> usize {
    data.iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_a() {
        assert_eq!(_a(&SAMPLE_DATA), 7);
    }

    #[test]
    fn test_b() {
        assert_eq!(_b(&SAMPLE_DATA), 5);
    }
}
