use std::collections::HashSet;

use glam::IVec3;
use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    let unalligned = parse(DATA);
    let (alligned, allignements) = allign_all_scanners(unalligned);
    println!("part a: {}", num_beacons(alligned));
    println!("part b: {}", greatest_manhatten_distance(allignements));
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

trait Rotatable {
    fn rot_z(self) -> Self;
    fn rot_y(self) -> Self;
    fn rot_x(self) -> Self;
}

impl Rotatable for IVec3 {
    fn rot_z(self) -> Self {
        Self::new(self.y, -self.x, self.z)
    }
    fn rot_y(self) -> Self {
        Self::new(-self.z, self.y, self.x)
    }
    fn rot_x(self) -> Self {
        Self::new(self.x, self.z, -self.y)
    }
}

fn rotation(mut p: IVec3, n: usize) -> IVec3 {
    p = match n / 4 {
        0 => p,
        1 => p.rot_x(),
        2 => p.rot_x().rot_x(),
        3 => p.rot_x().rot_x().rot_x(),
        4 => p.rot_y(),
        5 => p.rot_y().rot_y().rot_y(),
        _ => unreachable!(),
    };
    for _ in 0..(n % 4) {
        p = p.rot_z()
    }
    p
}

fn all_rotations(p: IVec3) -> impl Iterator<Item = IVec3> {
    (0..24).map(move |n| rotation(p, n))
}

fn dist_squared((&p1, &p2): (&IVec3, &IVec3)) -> i32 {
    let diff = p1 - p2;
    diff.x * diff.x + diff.y * diff.y + diff.z * diff.z
}

fn distances_squared(scan: &[IVec3]) -> HashSet<i32> {
    scan.iter().tuple_combinations().map(dist_squared).collect()
}

fn allign_all_scanners(unalligned: Vec<Vec<IVec3>>) -> (Vec<Vec<IVec3>>, Vec<IVec3>) {
    let distances = unalligned
        .iter()
        .map(|s| distances_squared(s))
        .collect_vec();
    let matches = distances
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((_, di), (_, dj))| di.intersection(dj).count() >= 66)
        .flat_map(|((i, _), (j, _))| [(i, j), (j, i)])
        .collect_vec();
    let mut alligned: Vec<Vec<IVec3>> = vec![vec![]; unalligned.len()];
    let mut allignements = vec![IVec3::new(0, 0, 0)];
    alligned[0] = unalligned[0].clone();
    while let Some(&(cur, next)) = matches
        .iter()
        .find(|&&(cur, next)| !alligned[cur].is_empty() && alligned[next].is_empty())
    {
        let common_distance = *distances[cur]
            .intersection(&distances[next])
            .next()
            .unwrap();
        let (&p1_alligned, &p2_alligned) = alligned[cur]
            .iter()
            .tuple_combinations()
            .find(|&pair| dist_squared(pair) == common_distance)
            .unwrap();
        let (&p1_unalligned, &p2_unalligned) = unalligned[next]
            .iter()
            .tuple_combinations()
            .find(|&pair| dist_squared(pair) == common_distance)
            .unwrap();
        let (n, (p1_corrected, p2_corrected)) = all_rotations(p1_unalligned)
            .zip(all_rotations(p2_unalligned))
            .find_position(|&(candidate1, candidate2)| {
                (candidate1 - p1_alligned == candidate2 - p2_alligned)
                    || (candidate2 - p1_alligned == candidate1 - p2_alligned)
            })
            .unwrap();
        let allignement = if p1_alligned - p1_corrected == p2_alligned - p2_corrected {
            p1_alligned - p1_corrected
        } else {
            assert_eq!(p2_alligned - p1_corrected, p1_alligned - p2_corrected);
            p2_alligned - p1_corrected
        };
        allignements.push(allignement);
        let new_alligned = unalligned[next]
            .iter()
            .map(|&p| rotation(p, n) + allignement)
            .collect();
        alligned[next] = new_alligned;
    }
    (alligned, allignements)
}

fn num_beacons(alligned: Vec<Vec<IVec3>>) -> usize {
    alligned.into_iter().flatten().unique().count()
}

fn greatest_manhatten_distance(allignements: Vec<IVec3>) -> usize {
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
    fn test_a() {
        let unalligned = parse(SAMPLE_DATA);
        let (alligned, _) = allign_all_scanners(unalligned);
        assert_eq!(num_beacons(alligned), 79);
    }

    #[test]
    fn test_b() {
        let unalligned = parse(SAMPLE_DATA);
        let (_, allignements) = allign_all_scanners(unalligned);
        assert_eq!(greatest_manhatten_distance(allignements), 3621);
    }
}
