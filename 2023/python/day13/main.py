import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import toolz  # noqa: F401

SAMPLE_INPUT = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def indices_of_reflection(note_row: str) -> set[int]:
    indices = set()
    for i in range(1, len(note_row)):
        for offset in range(min(len(note_row) - i, i)):
            if note_row[i + offset] != note_row[i - offset - 1]:
                break
        else:
            indices.add(i)
    # print(f"{note_row} -> {indices}")
    return indices


def indices_of_reflection_b(note_row: str) -> dict[int, int]:
    """Return number of fixes needed for each index to work"""
    indices = {}
    for i in range(1, len(note_row)):
        mismatches = 0
        for offset in range(min(len(note_row) - i, i)):
            if note_row[i + offset] != note_row[i - offset - 1]:
                mismatches += 1
        indices[i] = mismatches
    # print(f"{note_row} -> {indices}")
    return indices


def part_a(input: str):
    notes = [note.splitlines() for note in input.split("\n\n")]
    total = 0
    for note in notes:
        reflected_columns = set.intersection(
            *(indices_of_reflection(row) for row in note)
        )
        assert len(reflected_columns) <= 1
        if reflected_columns:
            (reflected_column,) = reflected_columns
            total += reflected_column
        reflection_rows = set.intersection(
            *(indices_of_reflection("".join(col)) for col in zip(*note))
        )
        assert len(reflection_rows) <= 1
        if reflection_rows:
            (reflected_row,) = reflection_rows
            total += 100 * reflected_row
        if not (reflected_columns or reflection_rows):
            for row in note:
                print(f"{row} -> {indices_of_reflection(row)}")
            raise ValueError("No reflection found:\n" + "\n".join(note))

    return total


def part_b(input: str):
    notes = [note.splitlines() for note in input.split("\n\n")]
    total = 0
    for note in notes:
        column_reflection_mismatches = [indices_of_reflection_b(row) for row in note]
        possible_reflected_columns = [
            i
            for i in range(1, len(note[0]))
            if sum(mismatches[i] for mismatches in column_reflection_mismatches) == 1
        ]
        assert len(possible_reflected_columns) <= 1
        if possible_reflected_columns:
            total += possible_reflected_columns[0]
        row_reflection_mismatches = [
            indices_of_reflection_b("".join(col)) for col in zip(*note)
        ]
        possible_reflected_rows = [
            i
            for i in range(1, len(note))
            if sum(mismatches[i] for mismatches in row_reflection_mismatches) == 1
        ]
        assert len(possible_reflected_rows) <= 1
        if possible_reflected_rows:
            total += 100 * possible_reflected_rows[0]
        if not (possible_reflected_columns or possible_reflected_rows):
            for row in note:
                print(f"{row} -> {indices_of_reflection(row)}")
            raise ValueError("No reflection found:\n" + "\n".join(note))

    return total


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 405
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 400
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
