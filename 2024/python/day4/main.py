import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import toolz  # noqa: F401


def part_a(input: str):
    lines = input.splitlines()
    total = 0
    look_for = [tuple("XMAS"), tuple(reversed("XMAS"))]
    # horizontal
    for line in lines:
        for word in toolz.sliding_window(4, line):
            if word in look_for:
                total += 1
    for a, b, c, d in toolz.sliding_window(4, lines):
        vertical = zip(a, b, c, d)
        # diagonal \
        diag1 = zip(a, b[1:], c[2:], d[3:])
        # diagonal /
        diag2 = zip(a[3:], b[2:], c[1:], d)
        for word in itertools.chain(vertical, diag1, diag2):
            if word in look_for:
                total += 1
    return total


def part_b(input: str):
    lines = input.splitlines()
    total = 0
    look_for = [tuple("MAS"), tuple(reversed("MAS"))]
    for a, b, c in toolz.sliding_window(3, lines):
        # diagonal \
        diags1 = zip(a, b[1:], c[2:])
        # diagonal /
        diags2 = zip(a[2:], b[1:], c)
        for diag1, diag2 in zip(diags1, diags2):
            if diag1 in look_for and diag2 in look_for:
                total += 1
    return total


SAMPLE_INPUT = """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = (Path(__file__).parent / "data.txt").read_text()


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 18
    with capsys.disabled():
        print(f"\nPart A: {part_a(INPUT)}", end=" ")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 9
    with capsys.disabled():
        print(f"\nPart B: {part_b(INPUT)}", end=" ")
