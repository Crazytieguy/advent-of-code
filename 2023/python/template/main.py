import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

SAMPLE_INPUT = """
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def part_a(input: str):
    return 0


def part_b(input: str):
    return 0


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 0
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 0
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
