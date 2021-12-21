#![allow(clippy::reversed_empty_ranges)]
use derive_new::new;
use itertools::{iterate, Itertools};
use ndarray::{s, Array, Array2, Zip};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(new)]
struct Image {
    data: Array2<bool>,
    default: bool,
}

fn parse(data: &'static str) -> (Vec<bool>, Image) {
    let lines = data.lines().collect_vec();
    let image_enhancement = lines[0].chars().map(|c| c == '#').collect_vec();
    let (rows, cols) = (lines[2..].len(), lines[2].len());
    let default = false;
    let mut data = Array2::from_elem([rows + 4, cols + 4], default);
    let data_vec = lines[2..]
        .iter()
        .flat_map(|line| line.chars().map(|c| c == '#'))
        .collect_vec();
    data.slice_mut(s![2..-2, 2..-2])
        .assign(&Array::from_shape_vec([rows, cols], data_vec).unwrap());
    (image_enhancement, Image::new(data, default))
}

fn enhance_image(image_enhancement: &[bool], image: &Image) -> Image {
    let shape = image.data.shape();
    let default = image_enhancement[if image.default { 0b111111111 } else { 0 }];
    let mut data = Array2::from_elem([shape[0] + 2, shape[1] + 2], default);
    Zip::from(image.data.windows([3, 3])).map_assign_into(
        data.slice_mut(s![2..-2, 2..-2]),
        |window| {
            let idx = (window[[0, 0]] as usize * 0b100000000)
                | (window[[0, 1]] as usize * 0b010000000)
                | (window[[0, 2]] as usize * 0b001000000)
                | (window[[1, 0]] as usize * 0b000100000)
                | (window[[1, 1]] as usize * 0b000010000)
                | (window[[1, 2]] as usize * 0b000001000)
                | (window[[2, 0]] as usize * 0b000000100)
                | (window[[2, 1]] as usize * 0b000000010)
                | (window[[2, 2]] as usize);
            image_enhancement[idx]
        },
    );
    Image::new(data, default)
}

fn solve(data: &'static str, iterations: usize) -> usize {
    let (image_enhancement, image) = parse(data);
    let image = iterate(image, |image| enhance_image(&image_enhancement, image))
        .nth(iterations)
        .unwrap();
    image.data.map(|&b| b as usize).sum()
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
