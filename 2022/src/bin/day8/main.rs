const DATA: &str = include_str!("data.txt");

fn parse(data: &str) -> Vec<&[u8]> {
    data.lines().map(|line| line.as_bytes()).collect()
}

fn iter_tree_directions<'a>(
    data: &'a [&'a [u8]],
) -> impl Iterator<
    Item = (
        u8,
        (
            impl ExactSizeIterator<Item = u8> + 'a,
            impl ExactSizeIterator<Item = u8> + 'a,
            impl ExactSizeIterator<Item = u8> + 'a,
            impl ExactSizeIterator<Item = u8> + 'a,
        ),
    ),
> + 'a {
    data.iter().enumerate().flat_map(move |(i, &row)| {
        row.iter().enumerate().map(move |(j, &tree)| {
            (
                tree,
                (
                    // to the right
                    row[j + 1..].iter().copied(),
                    // to the left
                    row[..j].iter().rev().copied(),
                    // to the bottom
                    data[i + 1..].iter().map(move |r| r[j]),
                    // to the top
                    data[..i].iter().rev().map(move |r| r[j]),
                ),
            )
        })
    })
}

fn part_a(data: &[&[u8]]) -> usize {
    iter_tree_directions(data)
        .map(|(tree, mut directions)| {
            let tree_is_higher = |other_tree| tree > other_tree;
            directions.0.all(tree_is_higher)
                || directions.1.all(tree_is_higher)
                || directions.2.all(tree_is_higher)
                || directions.3.all(tree_is_higher)
        })
        .filter(|&visible| visible)
        .count()
}

fn count_trees_visible(tree: u8, mut trees: impl ExactSizeIterator<Item = u8>) -> usize {
    let num_trees = trees.len();
    trees.position(|t| t >= tree).map_or(num_trees, |p| p + 1)
}

fn part_b(data: &[&[u8]]) -> usize {
    iter_tree_directions(data)
        .map(|(tree, directions)| {
            count_trees_visible(tree, directions.0)
                * count_trees_visible(tree, directions.1)
                * count_trees_visible(tree, directions.2)
                * count_trees_visible(tree, directions.3)
        })
        .max()
        .expect("At least one tree")
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 21);
        println!("part a: {}", part_a(&parse(DATA)));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 8);
        println!("part b: {}", part_b(&parse(DATA)));
    }
}

fn main() {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
}
