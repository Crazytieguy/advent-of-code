use advent_2022::*;
use itertools::izip;
use std::collections::VecDeque;

boilerplate!(Day);

impl Solution for Day {
    type Parsed = (Blizzards, usize);
    type A = usize;
    type B = usize;
    const SAMPLE_ANSWER_A: Self::TestA = 18;
    const SAMPLE_ANSWER_B: Self::TestB = 54;

    fn parse(data: &str) -> IResult<Self::Parsed> {
        let width = data.find('\n').expect("no newline") - 2;
        let (up, (down, (left, right))) = data
            .lines()
            .filter(|line| &line[2..3] != "#")
            .map(|line| {
                let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);
                line.bytes()
                    .filter(|&c| c != b'#')
                    .enumerate()
                    .for_each(|(col, c)| {
                        let bit = 1 << col;
                        match c {
                            b'>' => right |= bit,
                            b'<' => left |= bit,
                            b'^' => up |= bit,
                            b'v' => down |= bit,
                            _ => {}
                        };
                    });
                (up, (down, (left, right)))
            })
            .unzip();
        let blizzards = Blizzards {
            up,
            down,
            left,
            right,
        };
        Ok(("", (blizzards, width)))
    }

    fn parse_test(data: &'static str) -> IResult<Self::ParsedTest> {
        Self::parse(data)
    }

    fn a((mut blizzards, width): Self::Parsed) -> Self::A {
        simulate_shortest_path::<25>(&mut blizzards, width, Exit)
    }

    fn a_test((mut blizzards, width): Self::ParsedTest) -> Self::A {
        simulate_shortest_path::<4>(&mut blizzards, width, Exit)
    }

    fn b((mut blizzards, width): Self::Parsed) -> Self::B {
        simulate_3::<25>(&mut blizzards, width)
    }

    fn b_test((mut blizzards, width): Self::ParsedTest) -> Self::B {
        simulate_3::<4>(&mut blizzards, width)
    }
}

#[derive(Debug, Default, Clone)]
struct Blizzards {
    up: VecDeque<u128>,
    down: VecDeque<u128>,
    left: Vec<u128>,
    right: Vec<u128>,
}

fn mask(width: usize) -> u128 {
    (1 << width) - 1
}

impl Blizzards {
    fn update(&mut self, width: usize) {
        self.up.rotate_left(1);
        self.down.rotate_right(1);
        self.left.iter_mut().for_each(|row| {
            *row = (*row >> 1) | ((*row & 1) << (width - 1));
        });
        self.right.iter_mut().for_each(|row| {
            *row = (*row << 1) | (*row >> (width - 1));
            *row &= mask(width);
        });
    }
}

fn adjacent_positions<const HEIGHT: usize>(positions: &[u128], width: usize) -> [u128; HEIGHT] {
    let mut new_positions = [0; HEIGHT];
    for (row, above, cur, bellow) in izip!(
        &mut new_positions,
        [0].iter().chain(positions),
        positions,
        positions.iter().skip(1).chain([0].iter())
    ) {
        *row = (cur | cur << 1 | cur >> 1 | above | bellow) & mask(width);
    }
    new_positions
}

#[derive(Debug, Clone, Copy)]
enum Destination {
    Exit,
    Entrance,
}

use Destination::*;

fn simulate_3<const HEIGHT: usize>(blizzards: &mut Blizzards, width: usize) -> usize {
    simulate_shortest_path::<HEIGHT>(blizzards, width, Exit)
        + simulate_shortest_path::<HEIGHT>(blizzards, width, Entrance)
        + simulate_shortest_path::<HEIGHT>(blizzards, width, Exit)
}

fn simulate_shortest_path<const HEIGHT: usize>(
    blizzards: &mut Blizzards,
    width: usize,
    destination: Destination,
) -> usize {
    assert_eq!(HEIGHT, blizzards.right.len());
    let mut positions = [0; HEIGHT];
    for minute in 1.. {
        blizzards.update(width);
        positions = adjacent_positions(&positions, width);
        match destination {
            Exit => positions[0] |= 1,
            Entrance => positions[HEIGHT - 1] |= 1 << (width - 1),
        }
        for (p, up, down, left, right) in izip!(
            &mut positions,
            &blizzards.up,
            &blizzards.down,
            &blizzards.left,
            &blizzards.right
        ) {
            *p &= !(up | down | left | right);
        }
        if matches!(destination, Exit) && positions[HEIGHT - 1] >> (width - 1) == 1
            || matches!(destination, Entrance) && positions[0] & 1 == 1
        {
            blizzards.update(width);
            return minute + 1;
        }
    }
    unreachable!()
}
