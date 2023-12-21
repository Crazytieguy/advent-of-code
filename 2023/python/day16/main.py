import collections
from pathlib import Path

SAMPLE_INPUT = r""".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()

Queue = collections.deque[tuple[tuple[int, int], tuple[int, int]]]


def part_a(input: str, start=((0, 0), (0, 1))):
    grid = input.splitlines()
    seen = set()
    queue: Queue = collections.deque([start])
    while queue:
        beam = queue.popleft()
        if beam in seen:
            continue
        (row, col), (drow, dcol) = beam
        if row < 0 or col < 0 or row >= len(grid) or col >= len(grid[0]):
            continue
        seen.add(beam)
        match (drow, dcol):
            case (0, 1):
                match grid[row][col]:
                    case "|":
                        queue.append(((row + 1, col), (1, 0)))
                        queue.append(((row - 1, col), (-1, 0)))
                    case "/":
                        queue.append(((row - 1, col), (-1, 0)))
                    case "\\":
                        queue.append(((row + 1, col), (1, 0)))
                    case _:
                        queue.append(((row, col + 1), (0, 1)))
            case (0, -1):
                match grid[row][col]:
                    case "|":
                        queue.append(((row + 1, col), (1, 0)))
                        queue.append(((row - 1, col), (-1, 0)))
                    case "/":
                        queue.append(((row + 1, col), (1, 0)))
                    case "\\":
                        queue.append(((row - 1, col), (-1, 0)))
                    case _:
                        queue.append(((row, col - 1), (0, -1)))
            case (1, 0):
                match grid[row][col]:
                    case "-":
                        queue.append(((row, col + 1), (0, 1)))
                        queue.append(((row, col - 1), (0, -1)))
                    case "/":
                        queue.append(((row, col - 1), (0, -1)))
                    case "\\":
                        queue.append(((row, col + 1), (0, 1)))
                    case _:
                        queue.append(((row + 1, col), (1, 0)))
            case (-1, 0):
                match grid[row][col]:
                    case "-":
                        queue.append(((row, col + 1), (0, 1)))
                        queue.append(((row, col - 1), (0, -1)))
                    case "/":
                        queue.append(((row, col + 1), (0, 1)))
                    case "\\":
                        queue.append(((row, col - 1), (0, -1)))
                    case _:
                        queue.append(((row - 1, col), (-1, 0)))
    return len({coords for coords, _ in seen})


def part_b(input: str):
    grid = input.splitlines()
    rows = len(grid)
    cols = len(grid[0])
    max_energized = 0
    for row in range(rows):
        max_energized = max(max_energized, part_a(input, ((row, 0), (0, 1))))
        max_energized = max(max_energized, part_a(input, ((row, cols - 1), (0, -1))))
    for col in range(cols):
        max_energized = max(max_energized, part_a(input, ((0, col), (1, 0))))
        max_energized = max(max_energized, part_a(input, ((rows - 1, col), (-1, 0))))
    return max_energized


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 46
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 51
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
