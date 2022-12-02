const DATA: &str = include_str!("data.txt");

#[derive(Debug, Clone, Copy)]
enum Left {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy)]
enum Right {
    X,
    Y,
    Z,
}

type Parsed = Vec<(Left, Right)>;

fn parse(data: &'static str) -> Parsed {
    data.lines()
        .map(|line| {
            let (first, second) = line.split_once(' ').unwrap();
            (
                match first {
                    "A" => Left::A,
                    "B" => Left::B,
                    "C" => Left::C,
                    _ => panic!(),
                },
                match second {
                    "X" => Right::X,
                    "Y" => Right::Y,
                    "Z" => Right::Z,
                    _ => panic!(),
                },
            )
        })
        .collect()
}

fn choice_score(own: Right) -> usize {
    match own {
        Right::X => 1,
        Right::Y => 2,
        Right::Z => 3,
    }
}

fn part_a(data: &Parsed) -> usize {
    data.iter()
        .map(|&(opp, own)| {
            let result_score = match (opp, own) {
                (Left::A, Right::X) => 3,
                (Left::A, Right::Y) => 6,
                (Left::A, Right::Z) => 0,
                (Left::B, Right::Y) => 3,
                (Left::B, Right::Z) => 6,
                (Left::B, Right::X) => 0,
                (Left::C, Right::Z) => 3,
                (Left::C, Right::X) => 6,
                (Left::C, Right::Y) => 0,
            };
            choice_score(own) + result_score
        })
        .sum()
}

fn result_score(right: Right) -> usize {
    match right {
        Right::X => 0,
        Right::Y => 3,
        Right::Z => 6,
    }
}

fn part_b(data: &Parsed) -> usize {
    data.iter()
        .map(|&(opp, result)| {
            let choice_score = match (opp, result) {
                (Left::A, Right::Y) => 1,
                (Left::A, Right::Z) => 2,
                (Left::A, Right::X) => 3,
                (Left::B, Right::X) => 1,
                (Left::B, Right::Y) => 2,
                (Left::B, Right::Z) => 3,
                (Left::C, Right::Z) => 1,
                (Left::C, Right::X) => 2,
                (Left::C, Right::Y) => 3,
            };
            choice_score + result_score(result)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(&parse(SAMPLE_DATA)), 15);
        println!("part a: {}", part_a(&parse(DATA)));
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(&parse(SAMPLE_DATA)), 12);
        println!("part b: {}", part_b(&parse(DATA)));
    }
}

fn main() {
    let parsed = parse(DATA);
    println!("part a: {}", part_a(&parsed));
    println!("part b: {}", part_b(&parsed));
}
