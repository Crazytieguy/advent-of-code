use std::error::Error;

use nom::{
    character::complete::{char, line_ending, u8},
    multi::separated_list1,
    sequence::tuple,
    Parser,
};

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;
type IResult<'a, T> = nom::IResult<&'a str, T>;

type Tuple = (u8, u8, u8);
type Parsed<const N: usize> = (Vec<Tuple>, [[[bool; N]; N]; N]);

fn parse_cube(data: &str) -> IResult<(u8, u8, u8)> {
    tuple((u8, char(','), u8, char(','), u8))
        .map(|(a, _, b, _, c)| (a + 1, b + 1, c + 1))
        .parse(data)
}

fn parse<const N: usize>(data: &str) -> IResult<Parsed<N>> {
    let (input, coords) = separated_list1(line_ending, parse_cube)(data)?;
    let mut matrix = [[[false; N]; N]; N];
    for &(x, y, z) in &coords {
        matrix[x as usize][y as usize][z as usize] = true;
    }
    Ok((input, (coords, matrix)))
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

fn part_a<const N: usize>((coords, matrix): &Parsed<N>) -> usize {
    coords
        .iter()
        .flat_map(|&t| adjacent_coords::<N>(t))
        .filter(|&(x, y, z)| !matrix[x as usize][y as usize][z as usize])
        .count()
}

fn part_b<const N: usize>((_coords, matrix): &Parsed<N>) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a::<9>(&parse::<9>(SAMPLE_DATA)?.1), 64);
        println!("part a: {}", part_a::<24>(&parse::<24>(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b::<9>(&parse::<9>(SAMPLE_DATA)?.1), 58);
        println!("part b: {}", part_b::<24>(&parse::<24>(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse::<24>(DATA)?.1;
    println!("part a: {}", part_a::<24>(&parsed));
    println!("part b: {}", part_b::<24>(&parsed));
    Ok(())
}
