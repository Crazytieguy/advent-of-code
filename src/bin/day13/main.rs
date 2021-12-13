use itertools::{repeat_n, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("{}", part_b(DATA));
}

enum Fold {
    X(i64),
    Y(i64),
}

type Point = (i64, i64);

fn parse(data: &'static str) -> (Vec<Point>, Vec<Fold>) {
    let points = data
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect_vec();
    let folds = data
        .lines()
        .skip_while(|line| !line.starts_with('f'))
        .map(|line| {
            let (axis, value) = line.split_once('=').unwrap();
            let value = value.parse().unwrap();
            match axis {
                "fold along x" => Fold::X(value),
                "fold along y" => Fold::Y(value),
                _ => panic!(),
            }
        })
        .collect_vec();
    (points, folds)
}

fn part_a(data: &'static str) -> usize {
    let (points, folds) = parse(data);
    apply_fold(&folds[0], points).len()
}

fn apply_fold(fold: &Fold, points: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    match fold {
        Fold::X(v) => points
            .iter()
            .map(|&(x, y)| ((x - v).abs() - 1, y))
            .unique()
            .collect(),
        Fold::Y(v) => points
            .iter()
            .map(|&(x, y)| (x, v - (v - y).abs()))
            .unique()
            .collect(),
    }
}

#[allow(dead_code)]
fn part_b(data: &'static str) -> String {
    let (mut points, folds) = parse(data);
    points = folds
        .iter()
        .fold(points, |points, fold| apply_fold(fold, points));
    points.sort_unstable_by(|(xa, ya), (xb, yb)| (ya, xa).cmp(&(yb, xb)));
    let mut picture = String::new();
    let mut x = -1;
    let mut y = 0;
    for (next_x, next_y) in points {
        if next_y != y {
            picture.extend(repeat_n('\n', (next_y - y) as usize));
            x = -1;
        }
        picture.extend(repeat_n(' ', (next_x - x - 1) as usize));
        picture.push('#');
        y = next_y;
        x = next_x;
    }
    picture
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
        assert_eq!(part_b(SAMPLE_DATA), "#####\n#   #\n#   #\n#   #\n#####");
    }
}
