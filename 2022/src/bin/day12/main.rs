use std::collections::VecDeque;

use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

type Position = (usize, usize);
type HeightMap = Vec<Vec<u8>>;
struct Parsed {
    start: Position,
    end: Position,
    height_map: HeightMap,
}

fn parse(data: &str) -> Parsed {
    let mut height_map = data
        .lines()
        .map(|row| row.as_bytes().to_vec())
        .collect_vec();

    let mut find_position_and_assign = |from, to| {
        height_map
            .iter_mut()
            .enumerate()
            .find_map(|(x, row)| {
                row.iter_mut()
                    .enumerate()
                    .find(|(_, &mut height)| height == from)
                    .map(|(y, height)| {
                        *height = to;
                        (x, y)
                    })
            })
            .expect("The character should be present in the input")
    };

    Parsed {
        start: find_position_and_assign(b'S', b'a'),
        end: find_position_and_assign(b'E', b'z'),
        height_map,
    }
}

fn neighbors(height_map: &HeightMap, (x, y): Position) -> impl Iterator<Item = Position> + '_ {
    let checked_add_signed_2d =
        move |(dx, dy)| x.checked_add_signed(dx).zip(y.checked_add_signed(dy));
    let is_high_enough = move |&height| height >= height_map[x][y] - 1;
    let is_valid_neighbor = move |&(x, y): &Position| {
        height_map
            .get(x)
            .and_then(|row| row.get(y))
            .map_or(false, is_high_enough)
    };
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(checked_add_signed_2d)
        .filter(is_valid_neighbor)
}

fn bfs(
    Parsed {
        end, height_map, ..
    }: &Parsed,
    success: impl Fn(&Position) -> bool,
) -> usize {
    let mut seen = vec![vec![false; height_map[0].len()]; height_map.len()];
    let mut queue = VecDeque::from([(*end, 0)]);
    while let Some((pos, steps)) = queue.pop_front() {
        if success(&pos) {
            return steps;
        }
        if seen[pos.0][pos.1] {
            continue;
        }
        seen[pos.0][pos.1] = true;
        for neighbor in neighbors(height_map, pos) {
            queue.push_back((neighbor, steps + 1));
        }
    }
    unreachable!("should always find a path")
}

fn part_a(parsed: &Parsed) -> usize {
    bfs(parsed, |&pos| pos == parsed.start)
}

fn part_b(parsed: &Parsed) -> usize {
    bfs(parsed, |&(x, y)| parsed.height_map[x][y] == b'a')
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 31);
        println!("part a: {}", part_a(&parse(DATA)));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 29);
        println!("part b: {}", part_b(&parse(DATA)));
    }
}

fn main() {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
}
