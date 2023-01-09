#![feature(portable_simd)]
#![feature(array_windows)]
#![feature(get_many_mut)]
use advent_2022::*;
use itertools::{izip, Itertools};
use std::{ops::Range, simd::u8x32};

boilerplate!(Day);

impl BasicSolution for Day {
    type Parsed = BitGrid;
    type Answer = usize;
    const SAMPLE_ANSWER_A: Self::TestAnswer = 110;
    const SAMPLE_ANSWER_B: Self::TestAnswer = 20;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        let mut grid = BitGrid::new();
        data.lines().enumerate().for_each(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .for_each(|(col, _)| grid.insert(row + 24, col + 72))
        });
        Ok(("", grid))
    }

    fn a(mut elve_positions: Self::Parsed) -> Self::Answer {
        run_simulation(&mut elve_positions, 10);
        let (rows, cols) = elve_positions.bounds();
        rows.len() * cols.len() - elve_positions.len()
    }

    fn b(mut elve_positions: Self::Parsed) -> Self::Answer {
        run_simulation(&mut elve_positions, 10000).expect("not done within 10000 rounds")
    }
}

fn run_simulation(elve_positions: &mut BitGrid, max_rounds: usize) -> Option<usize> {
    for round in 0..max_rounds {
        let want_stay = elve_positions.want_stay();
        let mut want_north = elve_positions.want_north();
        let mut want_south = elve_positions.want_south();
        let mut want_west = elve_positions.want_west();
        let mut want_east = elve_positions.want_east();
        let mut prioritized_directions = [
            &mut want_north,
            &mut want_south,
            &mut want_west,
            &mut want_east,
        ];
        prioritized_directions.rotate_left(round % 4);
        for dir in &mut prioritized_directions {
            dir.difference_assign(&want_stay);
        }
        for i in 0..4 {
            for j in (i + 1)..4 {
                let [higher_priority, lower_priority] =
                    prioritized_directions.get_many_mut([i, j]).unwrap();
                lower_priority.difference_assign(higher_priority);
            }
        }
        cancel_north_south(&mut want_north, &mut want_south);
        cancel_west_east(&mut want_west, &mut want_east);
        if !(elve_positions.move_north(&want_north)
            | elve_positions.move_south(&want_south)
            | elve_positions.move_west(&want_west)
            | elve_positions.move_east(&want_east))
        {
            return Some(round + 1);
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BitGrid([u8x32; 160]);

fn shift_west(&row: &u8x32) -> u8x32 {
    (row >> u8x32::splat(1)) | (row.rotate_lanes_left::<1>() << u8x32::splat(7))
}

fn shift_east(&row: &u8x32) -> u8x32 {
    (row << u8x32::splat(1)) | (row.rotate_lanes_right::<1>() >> u8x32::splat(7))
}

fn cancel_west_east(want_west: &mut BitGrid, want_east: &mut BitGrid) {
    for (west, east) in izip!(&mut want_west.0, &mut want_east.0) {
        let both = shift_west(west) & shift_east(east);
        *west &= !shift_east(&both);
        *east &= !shift_west(&both);
    }
}

fn cancel_north_south(want_north: &mut BitGrid, want_south: &mut BitGrid) {
    for (north, south) in izip!(&mut want_north.0[2..], &mut want_south.0) {
        let both = *north & *south;
        *north &= !both;
        *south &= !both;
    }
}

impl BitGrid {
    fn new() -> Self {
        Self([Default::default(); 160])
    }

    fn move_north(&mut self, want_north: &Self) -> bool {
        let mut moved = false;
        for (i, &row) in want_north.0.iter().enumerate().skip(1) {
            if row != u8x32::splat(0) {
                moved = true;
                self.0[i] &= !row;
                self.0[i - 1] |= row;
            }
        }
        moved
    }

    fn move_south(&mut self, want_south: &Self) -> bool {
        let mut moved = false;
        for (i, &row) in want_south.0.iter().enumerate().rev().skip(1) {
            if row != u8x32::splat(0) {
                moved = true;
                self.0[i] &= !row;
                self.0[i + 1] |= row;
            }
        }
        moved
    }

    fn move_west(&mut self, want_west: &Self) -> bool {
        let mut moved = false;
        for (row, &west) in izip!(&mut self.0, &want_west.0) {
            if west != u8x32::splat(0) {
                moved = true;
                *row &= !west;
                *row |= shift_west(&west);
            }
        }
        moved
    }

    fn move_east(&mut self, want_east: &Self) -> bool {
        let mut moved = false;
        for (row, &east) in izip!(&mut self.0, &want_east.0) {
            if east != u8x32::splat(0) {
                moved = true;
                *row &= !east;
                *row |= shift_east(&east);
            }
        }
        moved
    }

    fn difference_assign(&mut self, other: &Self) {
        for (row, &other_row) in izip!(&mut self.0, &other.0) {
            *row &= !other_row;
        }
    }

    fn want_stay(&self) -> Self {
        let mut grid = self.clone();
        for (row, [above, cur, below]) in izip!(&mut grid.0[1..], self.0.array_windows()) {
            *row &= !(shift_west(above)
                | above
                | shift_east(above)
                | shift_west(cur)
                | shift_east(cur)
                | shift_west(below)
                | below
                | shift_east(below));
        }
        grid
    }

    fn want_north(&self) -> Self {
        let mut grid = self.clone();
        for (row, above) in izip!(&mut grid.0[1..], &self.0) {
            *row &= !(shift_west(above) | above | shift_east(above));
        }
        grid
    }

    fn want_south(&self) -> Self {
        let mut grid = self.clone();
        for (row, below) in izip!(&mut grid.0, &self.0[1..]) {
            *row &= !(shift_west(below) | below | shift_east(below));
        }
        grid
    }

    fn want_west(&self) -> Self {
        let mut grid = self.clone();
        for (row, [above, cur, below]) in izip!(&mut grid.0[1..], self.0.array_windows()) {
            *row &= !(shift_east(above) | shift_east(cur) | shift_east(below));
        }
        grid
    }

    fn want_east(&self) -> Self {
        let mut grid = self.clone();
        for (row, [above, cur, below]) in izip!(&mut grid.0[1..], self.0.array_windows()) {
            *row &= !(shift_west(above) | shift_west(cur) | shift_west(below));
        }
        grid
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
}

// for debugging
#[allow(dead_code)]
fn print_elve_positions(elves: &BitGrid) {
    let (rows, cols) = elves.bounds();
    for row in rows {
        for col in cols.clone() {
            if elves.get(row, col) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
