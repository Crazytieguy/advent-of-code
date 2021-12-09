use std::{
    collections::{BTreeSet, HashMap},
    iter::repeat,
};

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(Default)]
struct FourDigitDisplay {
    patterns: [&'static str; 10],
    output: [&'static str; 4],
}

impl From<&'static str> for FourDigitDisplay {
    fn from(s: &'static str) -> Self {
        let (patterns_str, output_str) = s.split(" | ").collect_tuple().unwrap();
        let mut fdd = Self::default();
        for (i, pat) in patterns_str.split_whitespace().enumerate() {
            fdd.patterns[i] = pat;
        }
        for (i, pat) in output_str.split_whitespace().enumerate() {
            fdd.output[i] = pat;
        }
        fdd
    }
}

fn part_a(data: &'static str) -> usize {
    data.lines()
        .map(FourDigitDisplay::from)
        .flat_map(|fdd| fdd.output)
        .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
        .count()
}

const PATTERNS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn get_algo() -> impl Fn(&[&'static str; 10]) -> HashMap<char, char> {
    let key_to_char_true = get_key_to_char(&PATTERNS);
    move |patterns| {
        let key_to_char_mangled = get_key_to_char(patterns);
        key_to_char_mangled
            .into_iter()
            .map(|(key, c)| (c, key_to_char_true[&key]))
            .collect()
    }
}

fn get_key_to_char(patterns: &[&'static str; 10]) -> HashMap<(usize, BTreeSet<usize>), char> {
    let char_to_occurences = patterns.iter().flat_map(|pat| pat.chars()).counts();
    let char_to_pat_lengths = patterns
        .iter()
        .flat_map(|pat| pat.chars().zip(repeat(pat.len())))
        .into_group_map();
    ('a'..='g')
        .map(|c| {
            (
                (
                    char_to_occurences[&c],
                    (&char_to_pat_lengths[&c]).iter().copied().collect(),
                ),
                c,
            )
        })
        .collect()
}

fn part_b(data: &'static str) -> usize {
    let decode = get_algo();
    data.lines()
        .map(FourDigitDisplay::from)
        .map(|fdd| {
            let translation = decode(&fdd.patterns);
            fdd.output
                .iter()
                .map(|&out_pat| {
                    let pat = out_pat
                        .chars()
                        .map(|c| translation[&c])
                        .sorted_unstable()
                        .collect::<String>();
                    PATTERNS
                        .iter()
                        .find_position(|&&p| p == pat)
                        .map(|(num, _)| num)
                        .unwrap()
                })
                .rev()
                .enumerate()
                .map(|(i, num)| num * 10_usize.pow(i as u32))
                .sum::<usize>()
        })
        .sum()
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
