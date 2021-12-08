use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(Default)]
struct FourDigitDisplay {
    patterns: [HashSet<char>; 10],
    output: [HashSet<char>; 4],
}

impl From<&'static str> for FourDigitDisplay {
    fn from(s: &'static str) -> Self {
        let (patterns_str, output_str) = s.split(" | ").collect_tuple().unwrap();
        let mut fdd = Self::default();
        for (i, pat) in patterns_str.split_whitespace().enumerate() {
            fdd.patterns[i] = pat.chars().collect();
        }
        for (i, pat) in output_str.split_whitespace().enumerate() {
            fdd.output[i] = pat.chars().collect();
        }
        fdd
    }
}

fn parse(data: &'static str) -> Vec<FourDigitDisplay> {
    data.lines().map(FourDigitDisplay::from).collect()
}

fn part_a(data: &'static str) -> usize {
    let displays = parse(data);
    let one_four_seven_eight = [2, 3, 4, 7];
    displays
        .iter()
        .flat_map(|fdd| fdd.output.iter())
        .filter(|digit| one_four_seven_eight.contains(&digit.len()))
        .count()
}

fn part_b(data: &'static str) -> usize {
    let mut sum = 0;
    for fdd in data.lines().map(FourDigitDisplay::from) {
        let mut digits = HashMap::new();
        for pat in &fdd.patterns {
            let digit = match pat.len() {
                2 => Some('1'),
                3 => Some('7'),
                4 => Some('4'),
                7 => Some('8'),
                _ => None,
            };
            if let Some(digit) = digit {
                digits.insert(digit, pat);
            }
        }
        for pat in &fdd.patterns {
            let digit = match pat.len() {
                5 => {
                    if pat.intersection(digits[&'4']).count() == 2 {
                        Some('2')
                    } else if pat.intersection(digits[&'1']).count() == 2 {
                        Some('3')
                    } else {
                        Some('5')
                    }
                }
                6 => {
                    if pat.intersection(digits[&'1']).count() == 1 {
                        Some('6')
                    } else if pat.intersection(digits[&'4']).count() == 3 {
                        Some('0')
                    } else {
                        Some('9')
                    }
                }
                _ => None,
            };
            if let Some(digit) = digit {
                digits.insert(digit, pat);
            }
        }
        sum += fdd
            .output
            .iter()
            .map(|out_pat| {
                digits
                    .keys()
                    .find(|digit| out_pat == digits[digit])
                    .unwrap()
            })
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 26);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 61229);
    }
}
