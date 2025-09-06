from itertools import pairwise
from pathlib import Path
from typing import Iterable

from toolz import peek


def part_a(input: str):
    return sum(map(is_safe_report, parse(input)))


def part_b(input: str):
    return sum(map(is_safe_report_dampened, parse(input)))


def parse(input: str):
    return ([int(n) for n in line.split()] for line in input.splitlines())


def is_safe_report_dampened(levels: list[int]) -> bool:
    return any(is_safe_report(skip(levels, i)) for i in range(len(levels)))


def is_safe_report(levels: Iterable[int]):
    first_diff, diffs = peek(b - a for a, b in pairwise(levels))

    def is_safe_diff(diff: int):
        return 1 <= abs(diff) <= 3 and diff * first_diff > 0

    return all(map(is_safe_diff, diffs))


def skip(seq: Iterable, skip: int):
    return (item for i, item in enumerate(seq) if i != skip)


SAMPLE_INPUT = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 2
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 4
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
