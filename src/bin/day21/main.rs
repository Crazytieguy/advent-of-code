use derive_new::new;
use itertools::Itertools;

const DATA: &str = include_str!("data.txt");

fn main() {
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));
}

fn parse(data: &'static str) -> (u32, u32) {
    data.lines()
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap())
        .collect_tuple()
        .unwrap()
}

fn part_a(data: &'static str) -> usize {
    let (mut p1_pos, mut p2_pos) = parse(data);
    let (mut p1_score, mut p2_score) = (0, 0);
    let mut die = (1..=100).cycle().enumerate();
    let mut turn = true;
    while p1_score < 1000 && p2_score < 1000 {
        let (pos, score) = match turn {
            true => {
                turn = false;
                (&mut p1_pos, &mut p1_score)
            }
            false => {
                turn = true;
                (&mut p2_pos, &mut p2_score)
            }
        };
        let roll_sum = die.by_ref().take(3).map(|(_i, roll)| roll).sum::<u32>();
        *pos = (*pos + roll_sum - 1) % 10 + 1;
        *score += *pos;
    }
    let num_rolls = die.next().map(|(i, _)| i).unwrap();
    num_rolls
        * match turn {
            true => p1_score,
            false => p2_score,
        } as usize
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

impl From<u32> for QuantumIterate {
    fn from(pos: u32) -> Self {
        Self {
            state: vec![PlayerState::new(pos as u8, 0, 1)],
        }
    }
}

impl Iterator for QuantumIterate {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_empty() {
            return None;
        }
        let (won, didnt_win) = self
            .state
            .iter()
            .flat_map(|&player_state| {
                [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].map(
                    |(roll_sum, num_universes)| {
                        let pos = (player_state.position + roll_sum - 1) % 10 + 1;
                        PlayerState::new(
                            pos,
                            player_state.score + pos,
                            player_state.num_universes * num_universes,
                        )
                    },
                )
            })
            .partition(|player_state| player_state.score >= 21);
        self.state = didnt_win;
        Some((
            won.iter()
                .map(|player_state| player_state.num_universes)
                .sum(),
            self.state
                .iter()
                .map(|player_state| player_state.num_universes)
                .sum(),
        ))
    }
}

fn part_b(data: &'static str) -> usize {
    let (p1_pos, p2_pos) = parse(data);
    let iterate_p1 = QuantumIterate::from(p1_pos);
    let iterate_p2 = QuantumIterate::from(p2_pos);
    let win_lose_per_turn_p1: Vec<_> = iterate_p1.collect();
    let win_lose_per_turn_p2: Vec<_> = iterate_p2.collect();
    let p1_wins = win_lose_per_turn_p1
        .iter()
        .skip(1)
        .zip(win_lose_per_turn_p2.iter())
        .map(|((wins_p1, _), (_, losses_p2))| wins_p1 * losses_p2)
        .sum();
    let p2_wins = win_lose_per_turn_p2
        .iter()
        .zip(win_lose_per_turn_p1.iter())
        .map(|((_, losses_p1), (wins_p2, _))| wins_p2 * losses_p1)
        .sum();
    if p1_wins > p2_wins {
        p1_wins
    } else {
        p2_wins
    }
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
