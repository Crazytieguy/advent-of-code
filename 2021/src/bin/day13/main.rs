use std::collections::HashSet;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("{}", part_b(DATA));
}

enum Fold {
    X(i64),
    Y(i64),
}

type Point = (usize, usize);

fn parse(data: &'static str) -> (HashSet<Point>, Vec<Fold>) {
    let mut lines = data.lines();
    let points = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds = lines
        .map(|line| {
            let (axis, value) = line.split_once('=').unwrap();
            let value = value.parse().unwrap();
            match axis {
                "fold along x" => Fold::X(value),
                "fold along y" => Fold::Y(value),
                _ => panic!(),
            }
        })
        .collect();
    (points, folds)
}

fn part_a(data: &'static str) -> usize {
    let (points, folds) = parse(data);
    apply_folds(points, &folds[0..=0]).len()
}

fn apply_folds(points: HashSet<Point>, folds: &[Fold]) -> HashSet<Point> {
    points
        .into_iter()
        .map(|p| {
            folds.iter().fold(p, |(x, y), fold| match *fold {
                Fold::X(v) => ((v - (v - x as i64).abs()) as usize, y),
                Fold::Y(v) => (x, (v - (v - y as i64).abs()) as usize),
            })
        })
        .collect()
}

fn part_b(data: &'static str) -> String {
    let (mut points, folds) = parse(data);
    points = apply_folds(points, &folds);
    let (max_x, max_y) = points
        .iter()
        .fold((0, 0), |(x1, y1), &(x2, y2)| (x1.max(x2), y1.max(y2)));
    let mut grid = vec![vec!['.'; max_x + 1]; max_y + 1];
    grid.iter_mut().for_each(|line| line.push('\n'));
    points.into_iter().for_each(|(x, y)| grid[y][x] = '█');
    grid.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 17);
    }

    #[test]
    fn test_b() {
        assert_eq!(
            part_b(SAMPLE_DATA),
            "█████\n\
             █...█\n\
             █...█\n\
             █...█\n\
             █████"
        );
    }
}
