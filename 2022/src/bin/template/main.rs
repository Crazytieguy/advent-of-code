use std::error::Error;

const DATA: &str = include_str!("data.txt");

type OutResult = std::result::Result<(), Box<dyn Error>>;

type IResult<'a, T> = nom::IResult<&'a str, T>;

type Parsed<'a> = &'a str;

fn parse(data: &str) -> IResult<Parsed> {
    Ok(("", data))
}

fn part_a(data: &Parsed) -> usize {
    println!("{data:?}");
    0
}

fn part_b(data: &Parsed) -> usize {
    println!("{data:?}");
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() -> OutResult {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 0);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> OutResult {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 0);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> OutResult {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
