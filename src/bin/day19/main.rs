use std::{
    collections::{HashMap, HashSet},
    iter::repeat_with,
};

use glam::IVec3;
use itertools::{iterate, Itertools};

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> Vec<Vec<IVec3>> {
    data.split("\n\n")
        .map(|scan| {
            scan.lines()
                .skip(1)
                .map(|line| {
                    let (x, y, z) = line
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    IVec3::new(x, y, z)
                })
                .collect()
        })
        .collect()
}

fn rotate_z(p: IVec3) -> IVec3 {
    IVec3::new(p.y, -p.x, p.z)
}

fn rotate_y(p: IVec3) -> IVec3 {
    IVec3::new(-p.z, p.y, p.x)
}

fn rotate_x(p: IVec3) -> IVec3 {
    IVec3::new(p.x, p.z, -p.y)
}

fn all_rotations(p: IVec3) -> impl Iterator<Item = IVec3> {
    iterate(p, |&p| rotate_z(p))
        .take(4)
        .flat_map(|p| iterate(p, |&p| rotate_x(p)).take(4))
        .flat_map(|p| iterate(p, |&p| rotate_y(p)).take(2))
}

fn all_scanner_variations(scanner: &[IVec3]) -> impl Iterator<Item = Vec<IVec3>> {
    let mut iter_vec = scanner.iter().copied().map(all_rotations).collect_vec();
    repeat_with(move || {
        iter_vec
            .iter_mut()
            .map(|it| it.next())
            .collect::<Option<Vec<IVec3>>>()
    })
    .while_some()
}

fn diffmap(scanner: &[IVec3]) -> HashMap<IVec3, Vec<(IVec3, IVec3)>> {
    scanner
        .iter()
        .tuple_combinations()
        .flat_map(|(&p1, &p2)| [(p1, p2), (p2, p1)])
        .into_group_map_by(|&(p1, p2)| p2 - p1)
}

// Assumes that a is aligned
fn compare_scanners(a: &[IVec3], b: &[IVec3]) -> Option<(Vec<IVec3>, IVec3)> {
    let a_diffmap = diffmap(a);
    all_scanner_variations(b).find_map(|mut b| {
        let b_diffmap = diffmap(&b);
        let alignment_counts = a_diffmap
            .iter()
            .flat_map(|(&diff, pairs)| {
                pairs
                    .iter()
                    .cartesian_product(b_diffmap.get(&diff).cloned().unwrap_or_default())
            })
            .flat_map(|(&(a1, a2), (b1, b2))| [(a1, b1), (a2, b2)])
            .unique()
            .map(|(a, b)| a - b)
            .counts();
        if let Some((alignment, _)) = alignment_counts.into_iter().find(|(_, count)| *count >= 12) {
            b.iter_mut().for_each(|p| *p += alignment);
            assert!(
                b.iter()
                    .collect::<HashSet<_>>()
                    .intersection(&a.iter().collect())
                    .count()
                    >= 12,
                "a:\n{:?}\nb:\n{:?}",
                a,
                b
            );
            Some((b, alignment))
        } else {
            None
        }
    })
}

fn part_a(data: &'static str) -> usize {
    let unalligned = parse(data);
    let (alligned, _) = allign_all_scanners(unalligned);
    alligned.into_iter().flatten().unique().count()
}

fn part_b(data: &'static str) -> usize {
    let unalligned = parse(data);
    let (_, allignements) = allign_all_scanners(unalligned);
    allignements
        .into_iter()
        .tuple_combinations()
        .map(|(p1, p2)| (p1 - p2).abs())
        .map(|dist| dist.x + dist.y + dist.z)
        .max()
        .unwrap() as usize
}

fn allign_all_scanners(mut unalligned: Vec<Vec<IVec3>>) -> (Vec<Vec<IVec3>>, Vec<IVec3>) {
    let mut alligned = vec![unalligned.remove(0)];
    let mut allignements = vec![IVec3::new(0, 0, 0)];
    while !unalligned.is_empty() {
        let (i, (alligned_scanner, allignement)) = alligned
            .iter()
            .cartesian_product(unalligned.iter().enumerate())
            .find_map(|(match_against, (i, scanner))| {
                Some(i).zip(compare_scanners(match_against, scanner))
            })
            .expect("no pair found :(");
        alligned.push(alligned_scanner);
        allignements.push(allignement);
        unalligned.remove(i);
    }
    (alligned, allignements)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_all_rotations() {
        assert_eq!(all_rotations(IVec3::new(1, 2, 3)).unique().count(), 24)
    }

    #[test]
    fn test_all_scanner_variations() {
        let scanner = parse(SAMPLE_DATA).remove(0);
        assert_eq!(all_scanner_variations(&scanner).unique().count(), 24)
    }

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 79);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 3621);
    }
}
