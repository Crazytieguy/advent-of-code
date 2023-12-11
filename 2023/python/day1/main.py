from pathlib import Path

SAMPLE_INPUT = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""
SAMPLE_INPUT_B = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"""
INPUT = Path("data.txt").read_text()


def part_a(input: str):
    return sum(extract_number(line) for line in input.splitlines())


def part_b(input: str):
    return sum(extract_number(fix_line(line)) for line in input.splitlines())


def fix_line(line: str):
    return (
        line.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
    )


def extract_number(line: str):
    first = ""
    last = ""
    for char in line:
        if char.isdigit():
            last = char
            if not first:
                first = char
    return int(first + last)


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 142
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 281
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
