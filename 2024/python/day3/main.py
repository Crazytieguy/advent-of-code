import re
from pathlib import Path

valid_mul = re.compile(r"mul\((\d+),(\d+)\)")


def part_a(input: str):
    total = 0
    for m in valid_mul.finditer(input):
        a, b = m.groups()
        total += int(a) * int(b)
    return total


valid_statement = re.compile(r"(do|don't)\(\)|mul\((\d+),(\d+)\)")


def part_b(input: str):
    enabled = True
    total = 0
    for m in valid_statement.finditer(input):
        a, b, c = m.groups()
        match a:
            case "do":
                enabled = True
            case "don't":
                enabled = False
            case None if enabled:
                total += int(b) * int(c)
    return total


SAMPLE_INPUT = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
SAMPLE_INPUT_B = (
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
)
INPUT = (Path(__file__).parent / "data.txt").read_text()


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 161
    with capsys.disabled():
        print(f"\nPart A: {part_a(INPUT)}", end=" ")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 48
    with capsys.disabled():
        print(f"\nPart B: {part_b(INPUT)}", end=" ")
