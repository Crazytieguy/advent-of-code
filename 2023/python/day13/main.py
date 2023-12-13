from pathlib import Path

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


def str_mismatches(a: str, b: str) -> int:
    return sum(ac != bc for ac, bc in zip(a, b))


def find_reflection_index(note: list[str], allowed_mismatches: int = 0) -> int | None:
    for i in range(1, len(note)):
        mirrored_row_pairs = zip(note[i:], reversed(note[:i]))
        mismatches = sum(str_mismatches(a, b) for a, b in mirrored_row_pairs)
        if mismatches == allowed_mismatches:
            return i
    return None


def score(note: list[str], allowed_mismatches: int = 0) -> int:
    if mirror_row := find_reflection_index(note, allowed_mismatches):
        return mirror_row * 100
    transposed = ["".join(col) for col in zip(*note)]
    if mirror_column := find_reflection_index(transposed, allowed_mismatches):
        return mirror_column
    raise ValueError("No reflection found:\n" + "\n".join(note))


def parse_input(input: str):
    return (note.splitlines() for note in input.split("\n\n"))


def part_a(input: str):
    return sum(score(note) for note in parse_input(input))


def part_b(input: str):
    return sum(score(note, 1) for note in parse_input(input))


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 405
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 400
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
