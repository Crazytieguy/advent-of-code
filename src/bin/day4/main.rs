use std::ops::Range;

const DATA: &str = include_str!("data.txt");
fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

#[derive(Debug)]
struct Board {
    numbers: Vec<usize>,
    marks: [bool; 25],
}

fn row_index_range(row: usize) -> Range<usize> {
    (row * 5)..(row * 5 + 5)
}

fn column_indexes(col: usize) -> Vec<usize> {
    (0..5).map(|row| row * 5 + col).collect()
}

impl Board {
    fn mark(&mut self, number: usize) {
        for (i, num) in self.numbers.iter().enumerate() {
            if *num == number {
                self.marks[i] = true;
            }
        }
    }

    fn is_winner(&self) -> bool {
        for i in 0..5 {
            if self.marks[row_index_range(i)].iter().all(|v| *v) {
                return true;
            }
            if column_indexes(i).iter().map(|j| self.marks[*j]).all(|v| v) {
                return true;
            }
        }
        false
    }

    fn score(&self, drawn: usize) -> usize {
        let sum_unmarked: usize = self
            .numbers
            .iter()
            .zip(self.marks.iter())
            .map(|(&num, &mark)| if !mark { num } else { 0 })
            .sum();
        sum_unmarked * drawn
    }
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let numbers = s.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let marks = [false; 25];
        Self { numbers, marks }
    }
}

fn parse(data: &'static str) -> (Vec<usize>, Vec<Board>) {
    let mut records = data.split("\n\n");
    let drawn = records
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards = records.map(Board::from).collect();
    (drawn, boards)
}

fn part_a(data: &'static str) -> usize {
    let (drawn, mut boards) = parse(data);
    for num in drawn {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.is_winner() {
                return board.score(num);
            }
        }
    }
    panic!()
}

#[allow(dead_code)]
fn part_b(data: &'static str) -> usize {
    let (drawn, mut boards) = parse(data);
    for num in drawn {
        for board in boards.iter_mut() {
            board.mark(num);
        }
        if boards.len() == 1 {
            assert!(boards[0].is_winner());
            return boards[0].score(num);
        }
        boards = boards.into_iter().filter(|b| !b.is_winner()).collect();
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 4512);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 1924);
    }
}
