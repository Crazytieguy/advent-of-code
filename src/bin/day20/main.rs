use std::collections::HashMap;

use derive_new::new;
use itertools::{iterate, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(new)]
struct Image {
    data: HashMap<(i64, i64), bool>,
    default: bool,
    bounds: ((i64, i64), (i64, i64)),
}

fn parse(data: &'static str) -> (Vec<bool>, Image) {
    let lines = data.lines().collect_vec();
    let image_enhancement = lines[0].chars().map(|c| c == '#').collect_vec();
    let data = lines[2..]
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((row as i64, col as i64), c == '#'))
        })
        .collect();
    let max_bound = (lines[2..].len() as i64, lines[2].len() as i64);
    (
        image_enhancement,
        Image::new(data, false, ((0, 0), max_bound)),
    )
}

fn bits_to_number(bits: &[bool]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &b)| (b as usize) << i)
        .sum()
}

fn enhance_image(image_enhancement: &[bool], image: &Image) -> Image {
    let ((min_row, min_col), (max_row, max_col)) = image.bounds;
    let data = ((min_row - 1)..=(max_row + 1))
        .cartesian_product((min_col - 1)..=(max_col + 1))
        .map(|(row, col)| {
            let bits = ((row - 1)..=(row + 1))
                .cartesian_product((col - 1)..=(col + 1))
                .map(|(row, col)| {
                    image
                        .data
                        .get(&(row, col))
                        .copied()
                        .unwrap_or(image.default)
                })
                .collect_vec();
            let enhanced_bit = image_enhancement[bits_to_number(&bits)];
            ((row, col), enhanced_bit)
        })
        .collect();
    let default = image_enhancement[bits_to_number(&[image.default; 9])];
    let new_bounds = ((min_row - 1, min_col - 1), (max_row + 1, max_col + 1));
    Image::new(data, default, new_bounds)
}

// for debugging
#[allow(dead_code)]
fn print_image(image: &Image) {
    let ((min_row, min_col), (max_row, max_col)) = image.bounds;
    let image_str: String = (min_row..=max_row)
        .flat_map(|row| {
            (min_col..=max_col)
                .map(move |col| if image.data[&(row, col)] { '#' } else { '.' })
                .chain(Some('\n'))
        })
        .collect();
    println!("{}", image_str)
}

fn solve(data: &'static str, iterations: usize) -> usize {
    let (image_enhancement, image) = parse(data);
    let image = iterate(image, |image| enhance_image(&image_enhancement, image))
        .nth(iterations)
        .unwrap();
    image.data.into_values().filter(|&b| b).count()
}

fn part_a(data: &'static str) -> usize {
    solve(data, 2)
}

fn part_b(data: &'static str) -> usize {
    solve(data, 50)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 35);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 3351);
    }
}
