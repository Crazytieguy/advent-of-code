import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from enum import Enum, auto
from pathlib import Path

import toolz  # noqa: F401


class Direction(Enum):
    UP = auto()
    DOWN = auto()
    LEFT = auto()
    RIGHT = auto()

    def rotate(self) -> "Direction":
        match self:
            case Direction.UP:
                return Direction.RIGHT
            case Direction.RIGHT:
                return Direction.DOWN
            case Direction.DOWN:
                return Direction.LEFT
            case Direction.LEFT:
                return Direction.UP


@dataclass(frozen=True)
class Position:
    y: int
    x: int

    def next(self, direction: Direction) -> "Position":
        match direction:
            case Direction.UP:
                return Position(self.y - 1, self.x)
            case Direction.DOWN:
                return Position(self.y + 1, self.x)
            case Direction.RIGHT:
                return Position(self.y, self.x + 1)
            case Direction.LEFT:
                return Position(self.y, self.x - 1)


def part_a(input: str):
    grid, guard_position = parse(input)
    direction = Direction.UP
    visited = {guard_position}
    while True:
        next_position = guard_position.next(direction)
        if (
            next_position.y == -1
            or next_position.y == len(grid)
            or next_position.x == -1
            or next_position.x == len(grid[0])
        ):
            return len(visited)
        match grid[next_position.y][next_position.x]:
            case "#":
                direction = direction.rotate()
            case "." | "^":
                guard_position = next_position
                visited.add(guard_position)


def part_b(input: str):
    grid, guard_position = parse(input)
    original_path, _ = find_path(grid, guard_position)
    obstructions = {position for position, _ in original_path}
    cause_a_loop = 0
    for obstruction in obstructions:
        _, is_loop = find_path(grid, guard_position, obstruction)
        if is_loop:
            cause_a_loop += 1
    return cause_a_loop


def find_path(
    grid: list[str], guard_position: Position, obstruction: Position | None = None
):
    direction = Direction.UP
    visited = set()
    while True:
        if (guard_position, direction) in visited:
            return visited, True
        visited.add((guard_position, direction))
        next_position = guard_position.next(direction)
        if (
            next_position.y == -1
            or next_position.y == len(grid)
            or next_position.x == -1
            or next_position.x == len(grid[0])
        ):
            return visited, False
        if next_position == obstruction:
            direction = direction.rotate()
            continue
        match grid[next_position.y][next_position.x]:
            case "#":
                direction = direction.rotate()
            case "." | "^":
                guard_position = next_position


def parse(input: str) -> tuple[list[str], Position]:
    grid = input.splitlines()
    for i, line in enumerate(grid):
        for j, c in enumerate(line):
            if c == "^":
                return grid, Position(y=i, x=j)
    raise ValueError("No guard")


SAMPLE_INPUT = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = (Path(__file__).parent / "data.txt").read_text()


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 41
    with capsys.disabled():
        print(f"\nPart A: {part_a(INPUT)}", end=" ")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 6
    with capsys.disabled():
        print(f"\nPart B: {part_b(INPUT)}", end=" ")
