import re
from collections import defaultdict
from dataclasses import dataclass
from itertools import chain
from pathlib import Path

SAMPLE_INPUT = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()

NUMBER = re.compile(r"\d+")


@dataclass
class Number:
    value: int
    row: int
    start_column: int
    end_column: int

    def adjacent_coords(self):
        return (
            (row, column)
            for row in range(self.row - 1, self.row + 2)
            for column in range(self.start_column - 1, self.end_column + 1)
        )


def part_a(input: str):
    grid = input.splitlines()
    numbers = parse_numbers(grid)
    return sum(
        number.value
        for number in numbers
        if any(
            ((c := get_2d(grid, row, column)) and c not in ".0123456789")
            for row, column in number.adjacent_coords()
        )
    )


def part_b(input: str):
    grid = input.splitlines()
    numbers = parse_numbers(grid)
    possible_gears = defaultdict(list)
    for number in numbers:
        for row, column in number.adjacent_coords():
            if get_2d(grid, row, column) == "*":
                possible_gears[(row, column)].append(number.value)
    return sum(vals[0] * vals[1] for vals in possible_gears.values() if len(vals) == 2)


def parse_numbers(grid: list[str]):
    return chain.from_iterable(
        find_numbers(line_num, line) for line_num, line in enumerate(grid)
    )


def find_numbers(line_num: int, line: str):
    return [
        Number(
            value=int(match.group(0)),
            row=line_num,
            start_column=match.start(),
            end_column=match.end(),
        )
        for match in NUMBER.finditer(line)
    ]


def get_2d(grid: list[str], row: int, column: int):
    if row < 0 or row >= len(grid):
        return None
    if column < 0 or column >= len(grid[row]):
        return None
    return grid[row][column]


def test_part_a(capsys):
    with capsys.disabled():
        assert part_a(SAMPLE_INPUT) == 4361
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 467835
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
