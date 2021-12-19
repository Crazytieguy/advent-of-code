use std::{collections::HashMap, iter::repeat_with};

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

fn all_scanner_variations(points: &[IVec3]) -> impl Iterator<Item = Vec<IVec3>> {
    let mut iter_vec = points.iter().copied().map(all_rotations).collect_vec();
    repeat_with(move || {
        iter_vec
            .iter_mut()
            .map(|it| it.next())
            .collect::<Option<Vec<IVec3>>>()
    })
    .while_some()
}

fn diffmap(points: &[IVec3]) -> HashMap<IVec3, Vec<(IVec3, IVec3)>> {
    points
        .iter()
        .tuple_combinations()
        .flat_map(|(&p1, &p2)| [(p1, p2), (p2, p1)])
        .into_group_map_by(|&(p1, p2)| p2 - p1)
}

struct ScannerData {
    points: Vec<IVec3>,
    diffmap: HashMap<IVec3, Vec<(IVec3, IVec3)>>,
}

impl ScannerData {
    fn new(points: Vec<IVec3>) -> Self {
        let diffmap = diffmap(&points);
        Self { points, diffmap }
    }
}

fn get_alignement(a: &ScannerData, b: &ScannerData) -> Option<IVec3> {
    let alignement_counts = a
        .diffmap
        .iter()
        .flat_map(|(&diff, pairs)| {
            pairs
                .iter()
                .cartesian_product(b.diffmap.get(&diff).cloned().unwrap_or_default())
        })
        .flat_map(|(&(a1, a2), (b1, b2))| [(a1, b1), (a2, b2)])
        .unique()
        .map(|(a, b)| a - b)
        .counts();
    alignement_counts
        .into_iter()
        .find(|(_, count)| *count >= 12)
        .map(|(alignement, _)| alignement)
}

fn allign_all_scanners(mut unalligned: Vec<Vec<IVec3>>) -> (Vec<Vec<IVec3>>, Vec<IVec3>) {
    let mut alligned = vec![ScannerData::new(unalligned.remove(0))];
    let mut allignements = vec![IVec3::new(0, 0, 0)];
    let mut unalligned_all_variations = unalligned
        .into_iter()
        .map(|points| {
            all_scanner_variations(&points)
                .map(ScannerData::new)
                .collect_vec()
        })
        .collect_vec();
    while !unalligned_all_variations.is_empty() {
        let ((scanner_idx, variation_idx), alignement) = alligned
            .iter()
            .rev()
            .flat_map(|match_against| {
                unalligned_all_variations.iter().enumerate().flat_map(
                    move |(scanner_idx, scanner_variations)| {
                        scanner_variations.iter().enumerate().map(
                            move |(variation_idx, scannerdata)| {
                                (match_against, scanner_idx, variation_idx, scannerdata)
                            },
                        )
                    },
                )
            })
            .find_map(|(match_against, scanner_idx, variation_idx, scannerdata)| {
                Some((scanner_idx, variation_idx)).zip(get_alignement(match_against, scannerdata))
            })
            .unwrap();
        alligned.push(ScannerData::new(
            unalligned_all_variations.remove(scanner_idx)[variation_idx]
                .points
                .iter()
                .map(|&p| p + alignement)
                .collect(),
        ));
        allignements.push(alignement);
    }
    (
        alligned.into_iter().map(|sd| sd.points).collect(),
        allignements,
    )
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
