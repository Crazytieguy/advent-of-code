#![feature(portable_simd)]
#![feature(iter_map_windows)]

use std::{ops::Range, simd::u8x32};

use advent_2022::*;
use itertools::{chain, Itertools};

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = BitGrid;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 110;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 20;

    fn parse(data: &str) -> IResult<'_, Self::Parsed> {
        let mut grid = BitGrid::new();
        data.lines().enumerate().for_each(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .for_each(|(col, _)| grid.insert(row + 24, col + 72))
        });
        Ok(("", grid))
    }

    fn a(mut elve_grid: Self::Parsed) -> Self::Answer {
        elve_grid.run_simulation(10);
        let (rows, cols) = elve_grid.bounds();
        rows.len() * cols.len() - elve_grid.len()
    }

    fn b(mut elve_grid: Self::Parsed) -> Self::Answer {
        elve_grid
            .run_simulation(10000)
            .expect("not done within 10000 rounds")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BitGrid([u8x32; 160]);

fn shift_west(&row: &u8x32) -> u8x32 {
    (row >> u8x32::splat(1)) | (row.rotate_elements_left::<1>() << u8x32::splat(7))
}

fn shift_east(&row: &u8x32) -> u8x32 {
    (row << u8x32::splat(1)) | (row.rotate_elements_right::<1>() >> u8x32::splat(7))
}

fn propose(
    [nw, n, ne]: &[u8x32; 3],
    [w, cur, e]: &[u8x32; 3],
    [sw, s, se]: &[u8x32; 3],
    priority: [Direction; 4],
) -> [u8x32; 4] {
    let mut propositions = [*cur; 4];
    let mut not_chosen = nw | n | ne | w | e | sw | s | se;
    for d in priority {
        let (row, dir_available) = match d {
            North => (&mut propositions[0], !(ne | n | nw)),
            South => (&mut propositions[1], !(se | s | sw)),
            West => (&mut propositions[2], !(nw | w | sw)),
            East => (&mut propositions[3], !(ne | e | se)),
        };
        *row &= dir_available & not_chosen;
        not_chosen &= !dir_available;
    }
    propositions
}

fn collide_proposals(
    [_, south, _, _]: &[u8x32; 4],
    [_, _, west, east]: &[u8x32; 4],
    [north, _, _, _]: &[u8x32; 4],
) -> [u8x32; 4] {
    [
        north & !*south,
        south & !*north,
        shift_west(west) & !shift_east(east),
        shift_east(east) & !shift_west(west),
    ]
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

use Direction::*;

impl BitGrid {
    fn new() -> Self {
        Self([Default::default(); 160])
    }

    fn run_simulation(&mut self, max_rounds: usize) -> Option<usize> {
        let mut priority = [North, South, West, East];
        for round in 0..max_rounds {
            let moved;
            (*self, moved) = self.play_round(priority);
            if !moved {
                return Some(round + 1);
            }
            priority.rotate_left(1);
        }
        None
    }

    fn play_round(&self, priority: [Direction; 4]) -> (Self, bool) {
        let mut new_self = self.clone();
        let mut moved = false;
        let zeros = [Default::default(); 2];
        chain!(&zeros, &self.0, &zeros)
            .map(|row| [shift_east(row), *row, shift_west(row)])
            .map_windows(|[above, cur, below]| propose(above, cur, below, priority))
            .map_windows(|[above, cur, below]| collide_proposals(above, cur, below))
            .enumerate()
            .for_each(|(i, [from_south, from_north, from_east, from_west])| {
                let destinations = from_north | from_south | from_west | from_east;
                if destinations == u8x32::splat(0) {
                    return;
                }
                moved = true;
                new_self.0[i + 1] &= !from_south;
                new_self.0[i - 1] &= !from_north;
                new_self.0[i] &= !shift_west(&from_west);
                new_self.0[i] &= !shift_east(&from_east);
                new_self.0[i] |= destinations;
            });
        (new_self, moved)
    }

    fn insert(&mut self, row: usize, col: usize) {
        self.0[row][col / 8] |= 1 << (col % 8);
    }

    fn get(&self, row: usize, col: usize) -> bool {
        self.0[row][col / 8] & (1 << (col % 8)) != 0
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..160)
            .cartesian_product(0..256)
            .filter(|&(row, col)| self.get(row, col))
    }

    fn bounds(&self) -> (Range<usize>, Range<usize>) {
        let mut min_row = usize::MAX;
        let mut max_row = usize::MIN;
        let mut min_col = usize::MAX;
        let mut max_col = usize::MIN;
        for (row, col) in self.iter() {
            min_row = min_row.min(row);
            max_row = max_row.max(row);
            min_col = min_col.min(col);
            max_col = max_col.max(col);
        }
        (min_row..max_row + 1, min_col..max_col + 1)
    }

    fn len(&self) -> usize {
        self.0
            .iter()
            .flat_map(|x| x.as_array())
            .map(|x| x.count_ones() as usize)
            .sum()
    }

    // for debugging
    #[allow(dead_code)]
    fn print(&self) {
        let (rows, cols) = self.bounds();
        for row in rows {
            for col in cols.clone() {
                if self.get(row, col) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}
