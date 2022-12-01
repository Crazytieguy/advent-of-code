use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> (Vec<u16>, usize) {
    (
        data.lines()
            .map(|line| u16::from_str_radix(line, 2).unwrap())
            .collect(),
        data.chars().take_while(|&c| c != '\n').count(),
    )
}

fn part_a(data: &'static str) -> usize {
    let (data, num_bits) = parse(data);
    let gamma = (0..num_bits)
        .map(|bit_pos| 1 << bit_pos)
        .filter(|mask| data.iter().filter(|&num| num & mask != 0).count() > data.len() / 2)
        .fold(0, |gamma, mask| gamma | mask) as usize;
    let interesting_bits = (1 << num_bits) - 1;
    gamma * (!gamma & interesting_bits)
}

fn part_b(data: &'static str) -> usize {
    let (data, num_bits) = parse(data);
    rating(&data, num_bits, true) * rating(&data, num_bits, false)
}

fn rating(data: &[u16], num_bits: usize, bit_criteria: bool) -> usize {
    let mut remaining = data.iter().copied().collect_vec();
    for bit_pos in (0..num_bits).rev() {
        let mask = 1 << bit_pos;
        let mut groups = remaining.into_iter().into_group_map_by(|num| num & mask);
        remaining = match bit_criteria == (groups[&mask].len() >= groups[&0].len()) {
            true => groups.remove(&mask).unwrap(),
            false => groups.remove(&0).unwrap(),
        };
        if remaining.len() == 1 {
            return remaining[0] as usize;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 198);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 230);
    }
}
