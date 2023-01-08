use advent_2022::*;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Vec<&'static [u8]>;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 21;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 8;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        Ok(("", data.lines().map(|line| line.as_bytes()).collect()))
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        iter_tree_directions(&data)
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

    fn b(data: Self::Parsed) -> Self::Answer {
        iter_tree_directions(&data)
            .map(|(tree, directions)| {
                count_trees_visible(tree, directions.0)
                    * count_trees_visible(tree, directions.1)
                    * count_trees_visible(tree, directions.2)
                    * count_trees_visible(tree, directions.3)
            })
            .max()
            .expect("At least one tree")
    }
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

fn count_trees_visible(tree: u8, mut trees: impl ExactSizeIterator<Item = u8>) -> usize {
    let num_trees = trees.len();
    trees
        .position(|other_tree| other_tree >= tree)
        .map_or(num_trees, |pos| pos + 1)
}
