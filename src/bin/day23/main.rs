const DATA: &str = include_str!("data.txt");
use std::collections::HashMap;

use arrayvec::ArrayVec;
use itertools::Itertools;
use Amphipod::*;

fn main() {
    println!("part b: {}", part_b(DATA));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost_multiplier(self) -> usize {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }

    fn room_id(self) -> usize {
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
        }
    }
}

type Room = ArrayVec<Amphipod, 4>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    hallway: [Option<Amphipod>; 7],
    rooms: [Room; 4],
}

impl State {
    fn room(&self, room_type: Amphipod) -> &Room {
        &self.rooms[room_type.room_id()]
    }

    fn room_mut(&mut self, room_type: Amphipod) -> &mut Room {
        &mut self.rooms[room_type.room_id()]
    }

    fn room_ready(&self, room_type: Amphipod) -> bool {
        self.room(room_type).iter().all(|&a| a == room_type)
    }

    fn steps_between(&self, hall_id: usize, room_type: Amphipod) -> Option<usize> {
        let room_id = room_type.room_id();
        let mut steps_taken = 1;
        for step in (hall_id + 1..room_id + 2).chain(room_id + 2..hall_id) {
            if self.hallway[step].is_some() {
                return None;
            }
            steps_taken += 2;
        }
        steps_taken += self.room(room_type).remaining_capacity();
        steps_taken += self.hallway[hall_id].is_none() as usize;
        steps_taken -= matches!(hall_id, 0 | 6) as usize;
        Some(steps_taken)
    }

    fn move_between(&self, hall_id: usize, room_type: Amphipod) -> Option<(Self, usize)> {
        match self.hallway[hall_id] {
            Some(a) => {
                if a != room_type || self.room(room_type).is_full() || !self.room_ready(room_type) {
                    None
                } else {
                    let steps_taken = self.steps_between(hall_id, room_type)?;
                    let mut new_state = self.clone();
                    new_state.hallway[hall_id] = None;
                    new_state.room_mut(room_type).push(room_type);
                    Some((new_state, steps_taken * room_type.cost_multiplier()))
                }
            }
            None => {
                if self.room(room_type).is_empty() || self.room_ready(room_type) {
                    None
                } else {
                    let steps_taken = self.steps_between(hall_id, room_type)?;
                    let mut new_state = self.clone();
                    let move_amphipod = new_state.room_mut(room_type).pop().unwrap();
                    new_state.hallway[hall_id] = Some(move_amphipod);
                    Some((new_state, steps_taken * move_amphipod.cost_multiplier()))
                }
            }
        }
    }
}

fn total_cost(state: State, memo: &mut HashMap<State, usize>) -> usize {
    if let Some(cost) = memo.get(&state) {
        return *cost;
    }
    if [A, B, C, D]
        .into_iter()
        .all(|room_type| state.room(room_type).as_slice() == [room_type; 4])
    {
        return 0;
    }
    let cost = (0..7)
        .cartesian_product([A, B, C, D].into_iter())
        .flat_map(|(hall_id, room_type)| state.move_between(hall_id, room_type))
        .map(|(new_state, move_cost)| move_cost.saturating_add(total_cost(new_state, memo)))
        .min()
        .unwrap_or(usize::MAX);
    memo.insert(state, cost);
    cost
}

fn parse(data: &'static str) -> State {
    let lns = data.lines().map(|l| l.as_bytes()).collect_vec();
    let hallway = [None; 7];
    let parse_amphipod = |c| match c {
        b'A' => A,
        b'B' => B,
        b'C' => C,
        b'D' => D,
        _ => panic!(),
    };
    let rooms = [
        [lns[5][3], lns[4][3], lns[3][3], lns[2][3]],
        [lns[5][5], lns[4][5], lns[3][5], lns[2][5]],
        [lns[5][7], lns[4][7], lns[3][7], lns[2][7]],
        [lns[5][9], lns[4][9], lns[3][9], lns[2][9]],
    ]
    .map(|byte_room| byte_room.map(parse_amphipod).into());
    State { hallway, rooms }
}

fn part_b(data: &'static str) -> usize {
    let state = parse(data);
    total_cost(state, &mut HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_DATA: &str = include_str!("sample.txt");

    #[test]
    fn test_steps_between() {
        let state = parse(SAMPLE_DATA);
        let (state, cost) = state.move_between(0, A).unwrap();
        assert_eq!(cost, 30);
        assert!(state.move_between(0, A).is_none());
        let (state, steps) = state.move_between(4, A).unwrap();
        assert_eq!(steps, 7000);
        let (state, steps) = state.move_between(1, A).unwrap();
        assert_eq!(steps, 4000);
        assert!(state.move_between(2, A).is_none());
    }

    #[test]
    fn test_b() {
        assert_eq!(part_b(SAMPLE_DATA), 44169);
    }
}
