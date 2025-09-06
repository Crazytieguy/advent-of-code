import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import toolz  # noqa: F401

SAMPLE_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def part_a(input: str):
    left, right = [], []
    for line in input.splitlines():
        l, r = line.split()
        left.append(int(l))
        right.append(int(r))
    left.sort()
    right.sort()
    return sum(abs(l - r) for l, r in zip(left, right))


def part_b(input: str):
    left, right = [], collections.Counter()
    for line in input.splitlines():
        l, r = line.split()
        left.append(int(l))
        right[int(r)] += 1
    left.sort()
    return sum(l * right[l] for l in left)


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 11
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 31
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
