use advent_2022::*;
use itertools::Itertools;
use std::collections::VecDeque;

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = Input;
    type A = u32;
    type B = u32;
    const SAMPLE_ANSWER_A: Self::TestA = 31;
    const SAMPLE_ANSWER_B: Self::TestB = 29;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
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
        Ok((
            "",
            Input {
                start: find_position_and_assign(b'S', b'a'),
                end: find_position_and_assign(b'E', b'z'),
                height_map,
            },
        ))
    }

    fn a(data: Self::Parsed) -> Self::A {
        bfs(&data, |&pos| pos == data.start)
    }

    fn b(data: Self::Parsed) -> Self::B {
        bfs(&data, |&(x, y)| data.height_map[x][y] == b'a')
    }
}

type Position = (usize, usize);
type HeightMap = Vec<Vec<u8>>;

#[derive(Debug, Clone)]
struct Input {
    start: Position,
    end: Position,
    height_map: HeightMap,
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
    Input {
        end, height_map, ..
    }: &Input,
    success: impl Fn(&Position) -> bool,
) -> u32 {
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
