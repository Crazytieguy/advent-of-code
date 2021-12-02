const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    // println!("part b: {}", b(DATA));
}

struct InputRecord;

impl From<&str> for InputRecord {
    fn from(s: &str) -> Self {
        todo!()
    }
}

fn part_a(data: &str) -> usize {
    for record in data.lines().map(InputRecord::from) {
        todo!()
    }
    0
}

#[allow(dead_code)]
fn part_b(data: &str) -> usize {
    for record in data.lines().map(InputRecord::from) {
        todo!()
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 0);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 0);
    }
}
