use advent_2022::*;
use nom::{
    character::complete::{char, line_ending, u8},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

boilerplate!(Day);

const SIZE: usize = 24;
const TEST_SIZE: usize = 9;

impl Solution for Day {
    type Parsed = (Vec<Tuple>, Arr3D<SIZE>);
    type ParsedTest = (Vec<Tuple>, Arr3D<TEST_SIZE>);
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 64;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 58;

    fn parse(data: &'static str) -> IResult<Self::Parsed> {
        parse(data)
    }

    fn a(data: Self::Parsed) -> Self::Answer {
        part_a(data)
    }

    fn b(data: Self::Parsed) -> Self::Answer {
        part_b(data)
    }

    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest> {
        parse(data)
    }

    fn a_test(data: Self::ParsedTest) -> Self::Answer {
        part_a(data)
    }

    fn b_test(data: Self::ParsedTest) -> Self::Answer {
        part_b(data)
    }
}

type Tuple = (u8, u8, u8);
type Arr3D<const N: usize> = [[[bool; N]; N]; N];

fn parse<const N: usize>(data: &str) -> IResult<(Vec<Tuple>, Arr3D<N>)> {
    separated_list1(line_ending, parse_cube)
        .map(|coords| {
            let mut matrix = [[[false; N]; N]; N];
            for &(x, y, z) in &coords {
                matrix[x as usize][y as usize][z as usize] = true;
            }
            (coords, matrix)
        })
        .parse(data)
}

fn part_a<const N: usize>((coords, matrix): (Vec<Tuple>, Arr3D<N>)) -> usize {
    coords
        .iter()
        .flat_map(|&t| adjacent_coords::<N>(t))
        .filter(|&(x, y, z)| !matrix[x as usize][y as usize][z as usize])
        .count()
}

fn part_b<const N: usize>((_coords, matrix): (Vec<Tuple>, Arr3D<N>)) -> usize {
    let mut visited = [[[false; N]; N]; N];
    let mut encountered = 0;
    let mut queue = vec![(0, 0, 0)];
    while let Some((x, y, z)) = queue.pop() {
        if visited[x as usize][y as usize][z as usize] {
            continue;
        }
        visited[x as usize][y as usize][z as usize] = true;
        adjacent_coords::<N>((x, y, z)).for_each(|(x, y, z)| {
            if matrix[x as usize][y as usize][z as usize] {
                encountered += 1;
            } else {
                queue.push((x, y, z))
            }
        });
    }
    encountered
}

fn parse_cube(data: &str) -> IResult<(u8, u8, u8)> {
    tuple((u8, char(','), u8, char(','), u8))
        .map(|(a, _, b, _, c)| (a + 1, b + 1, c + 1))
        .parse(data)
}

fn adjacent_coords<const N: usize>((x, y, z): Tuple) -> impl Iterator<Item = Tuple> {
    [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ]
    .iter()
    .filter_map(move |&(dx, dy, dz)| {
        x.checked_add_signed(dx)
            .zip(y.checked_add_signed(dy))
            .zip(z.checked_add_signed(dz))
    })
    .map(|((x, y), z)| (x, y, z))
    .filter(|&(x, y, z)| (x as usize) < N && (y as usize) < N && (z as usize) < N)
}
