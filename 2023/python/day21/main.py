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

    while steps > 0:
        steps -= 2
        to_add_even = []
        to_add_odd = []
        for row, col in frontier:
            for dr, dc in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                first_step_row = row + dr
                first_step_col = col + dc
                if first_step_row < 0 or first_step_row >= len(grid):
                    continue
                if first_step_col < 0 or first_step_col >= len(grid[0]):
                    continue
                if grid[first_step_row][first_step_col] == "#":
                    continue
                to_add_odd.append((first_step_row, first_step_col))
                for dr2, dc2 in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
                    second_step_row = first_step_row + dr2
                    second_step_col = first_step_col + dc2
                    if second_step_row < 0 or second_step_row >= len(grid):
                        continue
                    if second_step_col < 0 or second_step_col >= len(grid[0]):
                        continue
                    if grid[second_step_row][second_step_col] == "#":
                        continue
                    to_add_even.append((second_step_row, second_step_col))
        frontier = set(to_add_even) - even_reachable
        even_reachable.update(to_add_even)
        odd_reachable.update(to_add_odd)

    return len(even_reachable) if steps == 0 else len(odd_reachable)


def part_b(input: str, steps: int):
    grid = input.splitlines()
    n = steps // len(grid)
    count_full_odd = 1 + 2 * n + n**2
    count_full_even = n**2
    count_negative_odd_corners = n + 1
    count_positive_even_corners = n
    full_odd = part_a(input, len(grid) * 2 + 1)
    full_even = part_a(input, len(grid) * 2)
    half_odd = part_a(input, steps % len(grid))
    even_corners = [
        part_a(input, steps % len(grid) - 1, start)
        for start in [
            (0, 0),
            (0, len(grid) - 1),
            (len(grid) - 1, 0),
            (len(grid) - 1, len(grid) - 1),
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
        print(f"Part B: {part_b(INPUT, 26501365)}")
