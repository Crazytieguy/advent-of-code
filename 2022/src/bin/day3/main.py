import string
from collections.abc import Iterable
from pathlib import Path


def priority(item: str) -> int:
    return string.ascii_letters.index(item) + 1


def find_single_intersection(group: Iterable[str]) -> str:
    intersection = set.intersection(*(set(items) for items in group))
    assert len(intersection) == 1, group
    return intersection.pop()


def part_a(lines: list[str]) -> int:
    ans = 0
    for line in lines:
        left = line[: len(line) // 2]
        right = line[len(line) // 2 :]
        single_intersection = find_single_intersection([left, right])
        ans += priority(single_intersection)
    return ans


def part_b(lines: list[str]) -> int:
    ans = 0
    for i in range(0, len(lines), 3):
        group = lines[i : i + 3]
        single_intersection = find_single_intersection(group)
        ans += priority(single_intersection)
    return ans


def main():
    data = (Path(__file__).parent / "data.txt").read_text()
    lines = data.splitlines()
    print(f"part a: {part_a(lines)}")
    print(f"part b: {part_b(lines)}")


if __name__ == "__main__":
    main()
