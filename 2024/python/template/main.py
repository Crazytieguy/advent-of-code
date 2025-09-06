import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import toolz  # noqa: F401


def part_a(input: str):
    return 0


def part_b(input: str):
    return 0


SAMPLE_INPUT = """
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = (Path(__file__).parent / "data.txt").read_text()


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 0
    with capsys.disabled():
        print(f"\nPart A: {part_a(INPUT)}", end=" ")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 0
    with capsys.disabled():
        print(f"\nPart B: {part_b(INPUT)}", end=" ")
