use std::{collections::HashSet, error::Error};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

fn parse(data: &str) -> Vec<&[u8]> {
    data.lines().map(|line| line.as_bytes()).collect()
}

fn find_visible_trees(
    visible: &mut HashSet<(usize, usize)>,
    data: impl Iterator<Item = (usize, impl Iterator<Item = (usize, u8)>)>,
    row_length: usize,
) {
    let mut max_height_vertical = vec![0; row_length];
    for (i, row) in data {
        let mut max_height_horizontal = 0;
        for (j, tree) in row {
            if tree > max_height_horizontal || tree > max_height_vertical[j] {
                visible.insert((i, j));
            }
            max_height_horizontal = max_height_horizontal.max(tree);
            max_height_vertical[j] = max_height_vertical[j].max(tree);
        }
    }
}

fn part_a(data: &[&[u8]]) -> usize {
    let mut visible = HashSet::new();

    // iterate from top left
    find_visible_trees(
        &mut visible,
        data.iter()
            .map(|row| row.iter().copied().enumerate())
            .enumerate(),
        data[0].len(),
    );

    // iterate from bottom right
    find_visible_trees(
        &mut visible,
        data.iter()
            .map(|row| row.iter().copied().enumerate().rev())
            .enumerate()
            .rev(),
        data[0].len(),
    );

    visible.len()
}

fn count_visible_trees_from(mut trees: impl ExactSizeIterator<Item = u8>, tree: u8) -> usize {
    let num_trees = trees.len();
    trees.position(|t| t >= tree).map_or(num_trees, |p| p + 1)
}

fn part_b(data: &[&[u8]]) -> usize {
    data.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().map(move |(j, &tree)| {
                // to the right
                count_visible_trees_from(row[j + 1..].iter().copied(), tree)
                // to the left
                * count_visible_trees_from(row[..j].iter().rev().copied(), tree)
                // to the bottom
                * count_visible_trees_from(data[i + 1..].iter().map(|row| row[j]), tree)
                // to the top
                * count_visible_trees_from(data[..i].iter().rev().map(|row| row[j]), tree)
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
