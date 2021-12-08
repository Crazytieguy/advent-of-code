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

fn get_algo() -> impl Fn(&[HashSet<char>; 10]) -> HashMap<usize, &HashSet<char>> {
    let patterns = PATTERNS.map(|s| s.chars().collect::<HashSet<char>>());
    let positions_by_length = (0..10).into_group_map_by(|&i| patterns[i].len());
    let (determined, undetermined): (HashMap<_, _>, _) = positions_by_length
        .into_iter()
        .partition(|(_len, pats)| pats.len() == 1);
    let length_to_num: HashMap<_, _> = determined
        .into_iter()
        .map(|(len, positions)| (len, positions[0]))
        .collect();
    let positions_by_length_and_intersection_length = undetermined
        .into_iter()
        .flat_map(|(len, positions)| {
            positions
                .into_iter()
                .flat_map(|pos| {
                    length_to_num
                        .iter()
                        .map(|(&det_len, &det_pos)| {
                            (
                                (
                                    len,
                                    det_len,
                                    patterns[pos].intersection(&patterns[det_pos]).count(),
                                ),
                                pos,
                            )
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .into_group_map();
    let determined_second_order = positions_by_length_and_intersection_length
        .into_iter()
        .filter_map(|(cond, positions)| {
            if positions.len() == 1 {
                Some((positions[0], cond))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>()
        .into_iter()
        .map(|(k, (len, det_len, inter_len))| (len, (det_len, inter_len, k)))
        .into_group_map();
    move |patterns| {
        let positions_by_length = (0..10).into_group_map_by(|&i| patterns[i].len());
        let (determined, undetermined_): (HashMap<_, _>, _) = positions_by_length
            .into_iter()
            .partition(|(_len, pats)| pats.len() == 1);
        let determined: HashMap<_, _> = determined
            .into_iter()
            .map(|(len, positions)| (len, positions[0]))
            .collect();
        let mut decoded = determined
            .iter()
            .map(|(len, &pos)| (length_to_num[len], &patterns[pos]))
            .collect::<HashMap<_, _>>();
        decoded.extend(undetermined_.into_iter().flat_map(|(len, positions)| {
            positions
                .into_iter()
                .map(|pos| {
                    determined_second_order[&len]
                        .iter()
                        .find(|(det_len, inter_len, _num)| {
                            patterns[pos]
                                .intersection(&patterns[length_to_num[det_len]])
                                .count()
                                == *inter_len
                        })
                        .map(|(_, _, num)| (*num, &patterns[pos]))
                        .expect("number could not be determined")
                })
                .collect_vec()
        }));
        decoded
    }
}

fn part_b(data: &'static str) -> usize {
    let decode = get_algo();

    data.lines()
        .map(FourDigitDisplay::from)
        .map(|fdd| {
            let digits = decode(&fdd.patterns);
            fdd.output
                .iter()
                .map(|out_pat| {
                    digits
                        .keys()
                        .find(|digit| out_pat == digits[digit])
                        .unwrap()
                })
                .rev()
                .enumerate()
                .map(|(i, digit)| digit * 10_usize.pow(i as u32))
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
