use nom::IResult;

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

fn parse_unwrap(data: &'static str) -> Parsed {
    parse(data).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(&parse_unwrap(SAMPLE_DATA)), 0);
        println!("part a: {}", part_a(&parse_unwrap(DATA)));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&parse_unwrap(SAMPLE_DATA)), 0);
        println!("part b: {}", part_b(&parse_unwrap(DATA)));
    }
}

fn main() {
    let parsed = parse_unwrap(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
}
