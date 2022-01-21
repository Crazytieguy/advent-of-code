use itertools::Itertools;
use ndarray::Array2;

const DATA: &str = include_str!("data.txt");
fn main() {
    let (mut boards, draws) = parse(DATA);
    let win_turns = all_win_turns(&mut boards, &draws);
    println!("part a: {}", part_a_calc(&win_turns, &boards, &draws));
    println!("part b: {}", part_b_calc(&win_turns, &boards, &draws));
}

type Board = Array2<(u8, bool)>;

fn parse(data: &'static str) -> (Vec<Board>, Vec<u8>) {
    let mut split = data.trim().split("\n\n");
    let draws: Vec<u8> = split
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards = split
        .map(|board_str| {
            let board_vec = board_str
                .split_whitespace()
                .map(|n| (n.parse().unwrap(), false))
                .collect();
            Array2::from_shape_vec([5, 5], board_vec).unwrap()
        })
        .collect();
    (boards, draws)
}

fn mark_board(board: &mut Board, draw: u8) -> Option<(usize, usize)> {
    board
        .indexed_iter_mut()
        .find(|&(_idx, &mut (num, _mark))| num == draw)
        .map(|(idx, (_num, mark))| {
            *mark = true;
            idx
        })
}

fn board_win_turn(board: &mut Board, draws: &[u8]) -> usize {
    draws
        .iter()
        .position(|&draw| {
            if let Some((row, col)) = mark_board(board, draw) {
                return board.row(row).iter().all(|&(_num, mark)| mark)
                    || board.column(col).iter().all(|&(_num, mark)| mark);
            }
            false
        })
        .unwrap()
}

fn all_win_turns(boards: &mut [Board], draws: &[u8]) -> Vec<usize> {
    boards
        .iter_mut()
        .map(|b| board_win_turn(b, draws))
        .collect()
}

fn score(board: &Board, winning_draw: u8) -> usize {
    board
        .iter()
        .filter(|&&(_num, mark)| !mark)
        .map(|&(num, _mark)| num as usize)
        .sum::<usize>()
        * winning_draw as usize
}

fn part_a_calc(win_turns: &[usize], boards: &[Board], draws: &[u8]) -> usize {
    let best_board_id = win_turns.iter().position_min().unwrap();
    score(&boards[best_board_id], draws[win_turns[best_board_id]])
}

fn part_b_calc(win_turns: &[usize], boards: &[Board], draws: &[u8]) -> usize {
    let worst_board_id = win_turns.iter().position_max().unwrap();
    score(&boards[worst_board_id], draws[win_turns[worst_board_id]])
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_a() {
        let (mut boards, draws) = parse(SAMPLE_DATA);
        let win_turns = all_win_turns(&mut boards, &draws);
        assert_eq!(part_a_calc(&win_turns, &boards, &draws), 4512);
    }

    #[test]
    fn test_b() {
        let (mut boards, draws) = parse(SAMPLE_DATA);
        let win_turns = all_win_turns(&mut boards, &draws);
        assert_eq!(part_b_calc(&win_turns, &boards, &draws), 1924);
    }
}
