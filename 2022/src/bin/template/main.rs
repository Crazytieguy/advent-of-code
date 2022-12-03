use nom::IResult;
use std::error::Error;

const DATA: &str = include_str!("data.txt");

type Parsed = &'static str;

fn parse(data: &'static str) -> IResult<&'static str, Parsed> {
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
    fn test_a() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_a(&parse(SAMPLE_DATA)?.1), 0);
        println!("part a: {}", part_a(&parse(DATA)?.1));
        Ok(())
    }

    #[test]
    fn test_b() -> Result<(), Box<dyn Error>> {
        assert_eq!(part_b(&parse(SAMPLE_DATA)?.1), 0);
        println!("part b: {}", part_b(&parse(DATA)?.1));
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = parse(DATA)?.1;
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
    Ok(())
}
