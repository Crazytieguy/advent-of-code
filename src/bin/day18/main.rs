const DATA: &str = include_str!("data.txt");

use itertools::Itertools;

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Element {
    open_before: usize,
    closed_before: usize,
    val: usize,
}

fn parse(data: &'static str) -> Vec<Vec<Element>> {
    data.lines().map(parse_number).collect()
}

fn parse_number(number: &'static str) -> Vec<Element> {
    let groups = number.chars().group_by(|c| c.is_ascii_digit());
    groups
        .into_iter()
        .map(|(_, group)| group)
        .tuples()
        .map(|(non_digits, digits)| {
            let counts = non_digits.counts();
            Element {
                open_before: *counts.get(&'[').unwrap_or(&0),
                closed_before: *counts.get(&']').unwrap_or(&0),
                val: digits
                    .exactly_one()
                    .ok()
                    .and_then(|c| c.to_digit(10))
                    .unwrap() as usize,
            }
        })
        .collect()
}

fn reduce_snail_number(number: &mut Vec<Element>) {
    let mut nesting_level = 0;
    let explode_at = number
        .iter()
        .tuple_windows()
        .find_position(|(elem0, elem1)| {
            nesting_level += elem0.open_before;
            nesting_level -= elem0.closed_before;
            nesting_level > 4 && elem1.open_before + elem1.closed_before == 0
        })
        .map(|(i, _)| i);
    if let Some(i) = explode_at {
        if i > 0 {
            number[i - 1].val += number[i].val;
        }
        if i + 2 < number.len() {
            number[i + 2].val += number[i + 1].val;
            number[i + 2].closed_before -= 1;
        }
        number[i].val = 0;
        number[i].open_before -= 1;
        number.remove(i + 1);
        reduce_snail_number(number);
        return;
    }

    let split_at = number
        .iter()
        .find_position(|elem| elem.val >= 10)
        .map(|(i, _)| i);
    if let Some(i) = split_at {
        let cur_val = number[i].val;
        number[i].val /= 2;
        number.insert(
            i + 1,
            Element {
                open_before: 0,
                closed_before: 0,
                val: cur_val / 2 + cur_val % 2,
            },
        );
        number[i].open_before += 1;
        if i + 2 < number.len() {
            number[i + 2].closed_before += 1;
        }
        reduce_snail_number(number);
    }
}

fn get_magnitude(number: &[Element]) -> usize {
    let mut multiplier = 1;
    let mut magnitude = 0;
    for elem in number {
        multiplier *= 3_usize.pow(elem.open_before as u32);
        multiplier /= 2_usize.pow(elem.closed_before as u32);
        magnitude += multiplier * elem.val;
        multiplier *= 2;
        multiplier /= 3;
    }
    magnitude
}

fn part_a(data: &'static str) -> usize {
    let numbers = parse(data);
    let final_number = numbers.into_iter().reduce(sum_snail_numbers).unwrap();
    get_magnitude(&final_number)
}

fn sum_snail_numbers(mut a: Vec<Element>, mut b: Vec<Element>) -> Vec<Element> {
    b[0].closed_before = a
        .iter()
        .fold(0, |acc, elem| acc + elem.open_before - elem.closed_before);
    a[0].open_before += 1;
    a.append(&mut b);
    reduce_snail_number(&mut a);
    a
}

fn part_b(data: &'static str) -> usize {
    let numbers = parse(data);
    numbers
        .iter()
        .cartesian_product(numbers.iter())
        .flat_map(|(n0, n1)| [(n0, n1), (n1, n0)])
        .map(|(n0, n1)| {
            let mut summed = sum_snail_numbers(n0.clone(), n1.clone());
            reduce_snail_number(&mut summed);
            get_magnitude(&summed)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_parse_number() {
        assert_eq!(
            parse_number("[[1,2],3]"),
            [
                Element {
                    open_before: 2,
                    closed_before: 0,
                    val: 1
                },
                Element {
                    open_before: 0,
                    closed_before: 0,
                    val: 2
                },
                Element {
                    open_before: 0,
                    closed_before: 1,
                    val: 3
                },
            ]
        )
    }

    #[test]
    fn test_reduce_snail_number() {
        let mut number = parse_number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        reduce_snail_number(&mut number);
        assert_eq!(
            number.into_iter().map(|e| e.val).collect_vec(),
            [0, 7, 4, 7, 8, 6, 0, 8, 1]
        );
    }

    #[test]
    fn test_get_magnitue() {
        assert_eq!(get_magnitude(&parse_number("[[1,2],[[3,4],5]]")), 143);
        assert_eq!(
            get_magnitude(&parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            1384
        );
        assert_eq!(
            get_magnitude(&parse_number("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
            445
        );
        assert_eq!(
            get_magnitude(&parse_number("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
            791
        );
        assert_eq!(
            get_magnitude(&parse_number("[[[[5,0],[7,4]],[5,5]],[6,6]]")),
            1137
        );
        assert_eq!(
            get_magnitude(&parse_number(
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            )),
            3488
        );
    }

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 4140);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 3993);
    }
}
