use std::collections::HashMap;

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

lazy_static::lazy_static! {
    static ref PAIRS: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);
}

fn is_corrupt(stack: &mut Vec<char>, c: char) -> bool {
    if PAIRS.contains_key(&c) {
        stack.push(c);
        return false;
    }
    if let Some(last) = stack.pop() {
        if PAIRS[&last] == c {
            return false;
        }
    }
    true
}

fn part_a(data: &'static str) -> usize {
    let char_to_score = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    data.lines()
        .filter_map(|line| {
            let mut stack = Vec::new();
            line.chars().find(|&c| is_corrupt(&mut stack, c))
        })
        .map(|illegal_char| char_to_score[&illegal_char])
        .sum()
}

fn part_b(data: &'static str) -> usize {
    let char_to_score = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut scores = data
        .lines()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                if is_corrupt(&mut stack, c) {
                    return None;
                }
            }
            Some(stack)
        })
        .map(|stack| {
            stack
                .into_iter()
                .rev()
                .fold(0, |score, char| score * 5 + char_to_score[&char])
        })
        .collect_vec();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 26397);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 288957);
    }
}
