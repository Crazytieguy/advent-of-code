import string
from pathlib import Path

data = (Path(__file__).parent / "data.txt").read_text()


def priority(item: str) -> int:
    return string.ascii_letters.index(item) + 1


part_a = 0
for line in data.splitlines():
    left = line[: int(len(line) / 2)]
    right = line[int(len(line) / 2) :]
    intersection = set(left) & set(right)
    part_a += priority(intersection.pop())


print(f"{part_a=}")

part_b = 0
lines = data.splitlines()
i = 0
while i < len(lines):
    group = lines[i : i + 3]
    intersection = set.intersection(*(set(line) for line in group))
    part_b += priority(intersection.pop())
    i += 3

print(f"{part_b=}")
