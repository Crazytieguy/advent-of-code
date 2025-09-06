import collections
from pathlib import Path


def part_a(input: str):
    left, right = [], []
    for l, r in parse(input):
        left.append(l)
        right.append(r)
    left.sort()
    right.sort()
    return sum(abs(l - r) for l, r in zip(left, right))


def part_b(input: str):
    left, right = [], collections.Counter()
    for l, r in parse(input):
        left.append(l)
        right[r] += 1
    return sum(l * right[l] for l in left)


def parse(input: str):
    return (map(int, line.split()) for line in input.splitlines())


SAMPLE_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = (Path(__file__).parent / "data.txt").read_text()


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 11
    with capsys.disabled():
        print(f"\nPart A: {part_a(INPUT)}", end=" ")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 31
    with capsys.disabled():
        print(f"\nPart B: {part_b(INPUT)}", end=" ")
