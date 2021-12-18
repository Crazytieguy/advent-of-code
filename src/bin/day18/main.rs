const DATA: &str = include_str!("data.txt");

use derive_new::new;
use itertools::Itertools;

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, new)]
struct Element {
    open_before: usize,
    closed_before: usize,
    val: usize,
}

fn parse(data: &'static str) -> Vec<Vec<Element>> {
    data.lines().map(parse_number).collect()
}

fn parse_number(number: &'static str) -> Vec<Element> {
    number
        .split_inclusive(|c: char| c.is_ascii_digit())
        .flat_map(|elem| {
            elem.chars().last().and_then(|c| c.to_digit(10)).map(|val| {
                Element::new(
                    elem.chars().filter(|&c| c == '[').count(),
                    elem.chars().filter(|&c| c == ']').count(),
                    val as usize,
                )
            })
        })
        .collect()
}

fn reduce_snail_number(number: &mut Vec<Element>) {
    explode(number);
    split(number);
}

fn explode(number: &mut Vec<Element>) {
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
        number[i].open_before -= 1;
        if let Some(before_i) = i.checked_sub(1) {
            number[before_i].val += number[i].val;
        }
        number[i].val = 0;
        let removed = number.remove(i + 1);
        if let Some(after) = number.get_mut(i + 1) {
            after.val += removed.val;
            after.closed_before -= 1
        }
        explode(number);
    }
}

fn split(number: &mut Vec<Element>) {
    let split_at = number
        .iter()
        .find_position(|elem| elem.val >= 10)
        .map(|(i, _)| i);
    if let Some(i) = split_at {
        number[i].open_before += 1;
        if let Some(after) = number.get_mut(i + 1) {
            after.closed_before += 1
        }
        let new_val = number[i].val / 2 + number[i].val % 2;
        number.insert(i + 1, Element::new(0, 0, new_val));
        number[i].val /= 2;
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

fn sum_snail_numbers(mut a: Vec<Element>, mut b: Vec<Element>) -> Vec<Element> {
    b[0].closed_before = a
        .iter()
        .fold(0, |acc, elem| acc + elem.open_before - elem.closed_before);
    a[0].open_before += 1;
    a.append(&mut b);
    reduce_snail_number(&mut a);
    a
}

fn part_a(data: &'static str) -> usize {
    let numbers = parse(data);
    let final_number = numbers.into_iter().reduce(sum_snail_numbers).unwrap();
    get_magnitude(&final_number)
}

fn part_b(data: &'static str) -> usize {
    let numbers = parse(data);
    numbers
        .iter()
        .cartesian_product(numbers.iter())
        .flat_map(|(n0, n1)| [(n0, n1), (n1, n0)])
        .map(|(n0, n1)| get_magnitude(&sum_snail_numbers(n0.clone(), n1.clone())))
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
                Element::new(2, 0, 1),
                Element::new(0, 0, 2),
                Element::new(0, 1, 3),
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
        let test = |num, mag| assert_eq!(get_magnitude(&parse_number(num)), mag);
        test("[[1,2],[[3,4],5]]", 143);
        test("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        test("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        test("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        test("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        test(
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
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
