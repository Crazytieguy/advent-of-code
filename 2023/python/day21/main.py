from pathlib import Path

SAMPLE_INPUT = """\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def find_start(grid: list[str]) -> tuple[int, int]:
    for row, line in enumerate(grid):
        for col, char in enumerate(line):
            if char == "S":
                return (row, col)
    raise ValueError("No start found")


def part_a(input: str, steps: int, start: tuple[int, int] | None = None):
    grid = input.splitlines()
    if start is None:
        start = find_start(grid)

    even_reachable = {start}
    odd_reachable = set()
    frontier = even_reachable

    def coords_invalid(row, col):
        return (
            row < 0
            or row >= len(grid)
            or col < 0
            or col >= len(grid[0])
            or grid[row][col] == "#"
        )

    while steps > 0:
        steps -= 2
        to_add_even = []
        to_add_odd = []
        for row, col in frontier:
            for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                first_step_row = row + dr
                first_step_col = col + dc
                if coords_invalid(first_step_row, first_step_col):
                    continue
                to_add_odd.append((first_step_row, first_step_col))
                for dr2, dc2 in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                    second_step_row = first_step_row + dr2
                    second_step_col = first_step_col + dc2
                    if coords_invalid(second_step_row, second_step_col):
                        continue
                    to_add_even.append((second_step_row, second_step_col))
        frontier = set(to_add_even) - even_reachable
        even_reachable.update(to_add_even)
        odd_reachable.update(to_add_odd)

    return len(even_reachable) if steps == 0 else len(odd_reachable)


def part_b(input: str):
    steps = 26501365
    n = steps // 131  # number of full grid repetitions in each direction
    count_full_odd = 1 + 2 * n + n**2
    count_full_even = n**2
    count_negative_odd_corners = n + 1
    count_positive_even_corners = n
    full_odd = part_a(input, 263)  # a large odd number
    full_even = part_a(input, 262)  # a large even number
    half_odd = part_a(input, 65)
    even_corners = [
        part_a(input, 64, start)
        for start in [
            (0, 0),
            (0, 130),
            (130, 0),
            (130, 130),
        ]
    ]
    return (
        count_full_odd * full_odd
        + count_full_even * full_even
        - count_negative_odd_corners * (full_odd - half_odd)
        + sum(count_positive_even_corners * corner for corner in even_corners)
    )


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT, 6) == 16
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT, 64)}")


def test_part_b(capsys):
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
