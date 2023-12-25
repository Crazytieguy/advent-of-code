import itertools  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import pytest
import sympy

SAMPLE_INPUT = """19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


@dataclass
class HailStone:
    initial_position: tuple[int, int, int]
    velocity: tuple[int, int, int]

    def intersection_xy(self, other: "HailStone") -> tuple[float, float] | None:
        x1, y1, _ = self.initial_position
        x2, y2, _ = other.initial_position
        vx1, vy1, _ = self.velocity
        vx2, vy2, _ = other.velocity
        try:
            t2 = (y2 * vx1 - y1 * vx1 + x1 * vy1 - x2 * vy1) / (vx2 * vy1 - vy2 * vx1)
        except ZeroDivisionError:
            return None
        t1 = (x2 + t2 * vx2 - x1) / vx1
        if t1 < 0 or t2 < 0:
            return None
        x = x1 + t1 * vx1
        y = y1 + t1 * vy1
        return x, y


def find_stone_position(hailstones: list[HailStone]) -> tuple[int, int, int]:
    h1, h2, h3 = hailstones[:3]
    x1, y1, z1 = h1.initial_position
    x2, y2, z2 = h2.initial_position
    x3, y3, z3 = h3.initial_position
    v1x, v1y, v1z = h1.velocity
    v2x, v2y, v2z = h2.velocity
    v3x, v3y, v3z = h3.velocity
    x, y, z, vx, vy, vz = sympy.symbols("x y z vx vy vz", integer=True)
    t1, t2, t3 = sympy.symbols("t1 t2 t3", real=True)
    (solution,) = sympy.solve(
        [
            x + t1 * vx - x1 + t1 * v1x,
            y + t1 * vy - y1 + t1 * v1y,
            z + t1 * vz - z1 + t1 * v1z,
            x + t2 * vx - x2 + t2 * v2x,
            y + t2 * vy - y2 + t2 * v2y,
            z + t2 * vz - z2 + t2 * v2z,
            x + t3 * vx - x3 + t3 * v3x,
            y + t3 * vy - y3 + t3 * v3y,
            z + t3 * vz - z3 + t3 * v3z,
        ],
    )
    return solution[x], solution[y], solution[z]


def test_intersection_xy():
    h1 = HailStone((19, 13, 30), (-2, 1, -2))
    h2 = HailStone((18, 19, 22), (-1, -1, -2))
    assert h1.intersection_xy(h2) == pytest.approx((14.333, 15.333), 0.001)
    h4 = HailStone((20, 19, 15), (1, -5, -3))
    assert h1.intersection_xy(h4) is None
    h3 = HailStone((20, 25, 34), (-2, -2, -4))
    assert h2.intersection_xy(h3) is None


def parse_hailstone(line: str) -> HailStone:
    positions, velocities = line.split(" @ ")
    positions = tuple(map(int, positions.split(",")))
    velocities = tuple(map(int, velocities.split(",")))
    return HailStone(positions, velocities)  # type: ignore


def part_a(input: str, min_coord: int, max_coord: int):
    hailstones = [parse_hailstone(line) for line in input.splitlines()]
    total = 0
    for h1, h2 in itertools.combinations(hailstones, 2):
        if (intersection := h1.intersection_xy(h2)) is not None:
            x, y = intersection
            if min_coord <= x <= max_coord and min_coord <= y <= max_coord:
                total += 1
    return total


def part_b(input: str):
    hailstones = [parse_hailstone(line) for line in input.splitlines()]
    x, y, z = find_stone_position(hailstones)
    return x + y + z


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT, 7, 27) == 2
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT, 200000000000000, 400000000000000)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 47
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
