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


def mismatches_at(note_row: str, i: int) -> int:
    return sum(
        note_row[i + offset] != note_row[i - offset - 1]
        for offset in range(min(len(note_row) - i, i))
    )


def find_reflection_index(note: list[str], allowed_mismatches: int = 0) -> int | None:
    for i in range(1, len(note[0])):
        if sum(mismatches_at(row, i) for row in note) == allowed_mismatches:
            return i
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
