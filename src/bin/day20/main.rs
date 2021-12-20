use std::collections::HashMap;

use itertools::{iterate, repeat_n, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

struct Image {
    data: HashMap<(i64, i64), bool>,
    default: bool,
}

fn parse(data: &'static str) -> (Vec<bool>, Image) {
    let image_enhancement = data
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect_vec();

    let data = data
        .lines()
        .skip(2)
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((row as i64, col as i64), c == '#'))
        })
        .collect();
    (
        image_enhancement,
        Image {
            data,
            default: false,
        },
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
    let ((min_row, min_col), (max_row, max_col)) =
        image.data.keys().minmax().into_option().unwrap();
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
    let default_bits = repeat_n(image.default, 9).collect_vec();
    let default = image_enhancement[bits_to_number(&default_bits)];
    Image { data, default }
}

fn print_image(image: &Image) {
    let (&(min_row, min_col), &(max_row, max_col)) =
        image.data.keys().minmax().into_option().unwrap();
    let image_str: String = (min_row..=max_row)
        .flat_map(|row| {
            (min_col..=max_col)
                .map(move |col| if image.data[&(row, col)] { '#' } else { '.' })
                .chain(Some('\n'))
        })
        .collect();
    println!("{}", image_str)
}

fn part_a(data: &'static str) -> usize {
    let (image_enhancement, image) = parse(data);
    let image = iterate(image, |image| enhance_image(&image_enhancement, image))
        .inspect(print_image)
        .nth(2)
        .unwrap();
    image.data.into_values().filter(|&b| b).count()
}

fn part_b(data: &'static str) -> usize {
    let (image_enhancement, image) = parse(data);
    let image = iterate(image, |image| enhance_image(&image_enhancement, image))
        .inspect(|_| println!("Done iteration"))
        .nth(50)
        .unwrap();
    image.data.into_values().filter(|&b| b).count()
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
