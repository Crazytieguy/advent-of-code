use derive_new::new;
use num::Integer;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> [usize; 2] {
    let mut positions = data
        .lines()
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as usize);
    [positions.next().unwrap(), positions.next().unwrap()]
}

fn cycle<T: Integer>(val: T, quot: T) -> T {
    (val - T::one()) % quot + T::one()
}

fn part_a(data: &'static str) -> usize {
    let mut positions = parse(data);
    let mut scores = [0, 0];
    for turn in 0.. {
        let player_idx = turn % 2;
        let roll_sum = (1..=3).map(|i| cycle(turn * 3 + i, 100)).sum::<usize>();
        positions[player_idx] = cycle(positions[player_idx] + roll_sum, 10);
        scores[player_idx] += positions[player_idx];
        if scores[player_idx] >= 1000 {
            return scores[(player_idx + 1) % 2] * (turn + 1) * 3;
        }
    }
    unreachable!()
}

#[derive(new, Clone, Copy)]
struct PlayerState {
    position: u8,
    score: u8,
    num_universes: usize,
}

struct QuantumIterate {
    state: Vec<PlayerState>,
}

impl From<usize> for QuantumIterate {
    fn from(pos: usize) -> Self {
        Self {
            state: vec![PlayerState::new(pos as u8, 0, 1)],
        }
    }
}

impl Iterator for QuantumIterate {
    type Item = [usize; 2];

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_empty() {
            return None;
        }
        let (won, didnt_win) = self
            .state
            .iter()
            .flat_map(|&ps| {
                [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].map(
                    |(roll_sum, num_universes)| {
                        let pos = cycle(ps.position + roll_sum, 10);
                        PlayerState::new(pos, ps.score + pos, ps.num_universes * num_universes)
                    },
                )
            })
            .partition(|player_state| player_state.score >= 21);
        self.state = didnt_win;
        Some([&won, &self.state].map(|pss| {
            pss.iter()
                .map(|player_state| player_state.num_universes)
                .sum()
        }))
    }
}

fn part_b(data: &'static str) -> usize {
    let player_positions = parse(data);
    let player_win_lose_per_turn = player_positions
        .map(QuantumIterate::from)
        .map(|it| it.collect::<Vec<_>>());
    let [p1, p2] = player_win_lose_per_turn;
    let total_wins: [usize; 2] =
        [(&p1[1..], &p2[..]), (&p2[..], &p1[..])].map(|(winner, loser)| {
            winner
                .iter()
                .zip(loser.iter())
                .map(|([wins, _], [_, losses])| wins * losses)
                .sum()
        });
    total_wins[0].max(total_wins[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        assert_eq!(part_a(SAMPLE_DATA), 739785);
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 444_356_092_776_315);
    }
}
