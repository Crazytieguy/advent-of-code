use std::{cmp::Reverse, collections::BinaryHeap, error::Error};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

type Position = (usize, usize);
type HeightMap = Vec<Vec<u8>>;
type Parsed = (Position, Position, HeightMap);

fn parse(data: &str) -> Parsed {
    let mut starting_position = (0, 0);
    let mut ending_position = (0, 0);
    let height_map = data
        .lines()
        .enumerate()
        .map(|(x, row)| {
            row.bytes()
                .enumerate()
                .map(|(y, b)| match b {
                    b'S' => {
                        starting_position = (x, y);
                        b'a'
                    }
                    b'E' => {
                        ending_position = (x, y);
                        b'z'
                    }
                    _ => b,
                })
                .collect()
        })
        .collect();
    (starting_position, ending_position, height_map)
}

fn part_a((start, end, height_map): &Parsed) -> usize {
    let mut minimum_steps_to = vec![vec![None::<usize>; height_map[0].len()]; height_map.len()];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), *start));
    while let Some((Reverse(steps), (x, y))) = queue.pop() {
        if (x, y) == *end {
            return steps;
        }
        if let Some(minimum_steps) = minimum_steps_to[x][y] {
            if steps >= minimum_steps {
                continue;
            }
        }
        minimum_steps_to[x][y] = Some(steps);
        let height = height_map[x][y];
        let neighbors = [
            if y < height_map[0].len() - 1 {
                Some((x, y + 1))
            } else {
                None
            },
            if y > 0 { Some((x, y - 1)) } else { None },
            if x < height_map.len() - 1 {
                Some((x + 1, y))
            } else {
                None
            },
            if x > 0 { Some((x - 1, y)) } else { None },
        ];
        for (x, y) in neighbors.iter().flatten().copied() {
            if height_map[x][y] <= height + 1 {
                queue.push((Reverse(steps + 1), (x, y)));
            }
        }
    }
    unreachable!("no path found")
}

fn part_b((_start, end, height_map): &Parsed) -> usize {
    let mut minimum_steps_to = vec![vec![None::<usize>; height_map[0].len()]; height_map.len()];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), *end));
    let mut best = usize::MAX;
    while let Some((Reverse(steps), (x, y))) = queue.pop() {
        let height = height_map[x][y];
        if height == b'a' {
            best = best.min(steps);
            continue;
        }
        if let Some(minimum_steps) = minimum_steps_to[x][y] {
            if steps >= minimum_steps {
                continue;
            }
        }
        minimum_steps_to[x][y] = Some(steps);
        let neighbors = [
            if y < height_map[0].len() - 1 {
                Some((x, y + 1))
            } else {
                None
            },
            if y > 0 { Some((x, y - 1)) } else { None },
            if x < height_map.len() - 1 {
                Some((x + 1, y))
            } else {
                None
            },
            if x > 0 { Some((x - 1, y)) } else { None },
        ];
        for (x, y) in neighbors.iter().flatten().copied() {
            if height <= height_map[x][y] + 1 {
                queue.push((Reverse(steps + 1), (x, y)));
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 31);
        println!("part a: {}", part_a(&parse(DATA)));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 29);
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
