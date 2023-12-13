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


def count_mismatches_around_each_index(note_row: str) -> list[int]:
    return [
        sum(
            note_row[i + offset] != note_row[i - offset - 1]
            for offset in range(min(len(note_row) - i, i))
        )
        for i in range(1, len(note_row))
    ]


def find_reflection_index(note: list[str], allowed_mismatches: int = 0) -> int | None:
    reflection_mismatches = [count_mismatches_around_each_index(row) for row in note]
    for i, mismatch_counts in enumerate(zip(*reflection_mismatches)):
        if sum(mismatch_counts) == allowed_mismatches:
            return i + 1
    return None


def score(note: list[str], allowed_mismatches: int = 0) -> int:
    if mirror_column := find_reflection_index(note, allowed_mismatches):
        return mirror_column
    transposed = ["".join(col) for col in zip(*note)]
    if mirror_row := find_reflection_index(transposed, allowed_mismatches):
        return 100 * mirror_row
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
