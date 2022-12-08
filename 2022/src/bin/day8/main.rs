use std::{collections::HashSet, error::Error};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

fn parse(data: &str) -> Vec<&[u8]> {
    data.lines().map(|line| line.as_bytes()).collect()
}

fn part_a(data: &[&[u8]]) -> usize {
    let mut visible = HashSet::new();
    let mut max_height_from_top = vec![0; data[0].len()];
    data.iter().enumerate().for_each(|(i, row)| {
        let mut max_height_from_left = 0;
        row.iter().enumerate().for_each(|(j, &tree)| {
            if tree > max_height_from_left || tree > max_height_from_top[j] {
                visible.insert((i, j));
            }
            max_height_from_left = max_height_from_left.max(tree);
            max_height_from_top[j] = max_height_from_top[j].max(tree);
        });
    });
    let mut max_height_from_bottom = vec![0; data[0].len()];
    data.iter().enumerate().rev().for_each(|(i, row)| {
        let mut max_height_from_right = 0;
        row.iter().enumerate().rev().for_each(|(j, &tree)| {
            if tree > max_height_from_right || tree > max_height_from_bottom[j] {
                visible.insert((i, j));
            }
            max_height_from_right = max_height_from_right.max(tree);
            max_height_from_bottom[j] = max_height_from_bottom[j].max(tree);
        });
    });
    visible.len()
}

fn count_visible_trees(mut trees: impl ExactSizeIterator<Item = u8>, tree: u8) -> usize {
    let num_trees = trees.len();
    trees.position(|t| t >= tree).map_or(num_trees, |p| p + 1)
}

fn part_b(data: &[&[u8]]) -> usize {
    data.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, &tree)| {
                // to the right
                count_visible_trees(row[j + 1..].iter().copied(), tree)
                // to the left
                * count_visible_trees(row[..j].iter().rev().copied(), tree)
                // to the bottom
                * count_visible_trees(data[i + 1..].iter().map(|row| row[j]), tree)
                // to the top
                * count_visible_trees(data[..i].iter().rev().map(|row| row[j]), tree)
            })
        })
        .max()
        .expect("At least one tree")
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 21);
        println!("part a: {}", part_a(&parse(DATA)));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 8);
        println!("part b: {}", part_b(&parse(DATA)));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
