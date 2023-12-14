import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import toolz  # noqa: F401

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


def roll_start(row: str) -> str:
    new_row = []
    for i, c in enumerate(row):
        if c == "O":
            new_row.append("O")
        if c == "#":
            new_row.extend("." * (i - len(new_row)))
            new_row.append("#")
    new_row.extend("." * (len(row) - len(new_row)))
    return "".join(new_row)


def test_roll_start():
    assert roll_start(".O#..O.#.#") == "O.#O...#.#"


def cycle(grid: list[str]) -> list[str]:
    # grid starts north -> south x west -> east
    # tilt north
    grid = [roll_start("".join(row)) for row in zip(*grid)]
    grid = ["".join(row) for row in zip(*grid)]
    # tilt west
    grid = [roll_start(row) for row in grid]
    # tilt south
    grid = [roll_start("".join(reversed(row))) for row in zip(*grid)]
    grid = ["".join(row) for row in zip(*grid)][::-1]
    # tilt east
    grid = ["".join(reversed(roll_start("".join(reversed(row))))) for row in grid]
    return grid


def part_a(input: str):
    rows = input.splitlines()
    columns = list(zip(*rows))
    total = 0
    for column in columns:
        load = len(column)
        for i, c in enumerate(column):
            if c == "O":
                total += load
                load -= 1
            if c == "#":
                load = len(column) - i - 1
    return total


def find_last_grid(initial_grid: list[str]) -> list[str]:
    grid_history = [initial_grid]
    for cycles in range(1000):
        new_grid = cycle(grid_history[-1])
        for i, grid in enumerate(grid_history):
            if grid == new_grid:
                start = i
                length = cycles + 1 - i
                offset = (1_000_000_000 - start) % length
                return grid_history[start + offset]
        grid_history.append(new_grid)
    raise RuntimeError("No match found")


def part_b(input: str):
    grid = input.splitlines()
    last_grid = find_last_grid(grid)
    columns = list(zip(*last_grid))
    total = 0
    for column in columns:
        max_load = len(column)
        for i, c in enumerate(column):
            if c == "O":
                total += max_load - i
    return total


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 136
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 64
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
