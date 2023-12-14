from pathlib import Path
from typing import Iterable

SAMPLE_INPUT = """\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def part_a(input: str):
    grid = input.splitlines()
    tilted_north = list(zip(*map(roll_start, zip(*grid))))
    return count_north_beam_load(tilted_north)


def part_b(input: str):
    grid = input.splitlines()
    billionth_grid = find_billionth_grid(grid)
    return count_north_beam_load(billionth_grid)


def find_billionth_grid(initial_grid: list[str]) -> list[str]:
    grid_history = [initial_grid]
    for num_cycles in range(1, 1000):
        next_grid = spin_cycle(grid_history[-1])
        for i, grid in enumerate(grid_history):
            if grid == next_grid:
                start = i
                length = num_cycles - start
                offset = (1_000_000_000 - start) % length
                return grid_history[start + offset]
        grid_history.append(next_grid)
    raise RuntimeError("No match found")


def spin_cycle(grid: list[str]) -> list[str]:
    tilted_north = zip(*map(roll_start, zip(*grid)))
    tilted_west = map(roll_start, tilted_north)
    tilted_south = reversed(
        list(zip(*(roll_start(reversed(row)) for row in zip(*tilted_west))))
    )
    tilted_east = ["".join(reversed(roll_start(reversed(row)))) for row in tilted_south]
    return tilted_east


def roll_start(row: Iterable[str]) -> str:
    new_row = ""
    for i, c in enumerate(row):
        if c == "O":
            new_row += "O"
        if c == "#":
            new_row += "." * (i - len(new_row))
            new_row += "#"
    new_row += "." * (i + 1 - len(new_row))  # type: ignore
    return new_row


def test_roll_start():
    assert roll_start(".O#..O.#.#") == "O.#O...#.#"


def count_north_beam_load(grid: list[str]):
    return sum(map(count_column_load, zip(*grid)))


def count_column_load(column: str):
    return sum(i + 1 for i, c in enumerate(reversed(column)) if c == "O")


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 136
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 64
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
