import collections  # noqa: F401
import itertools  # noqa: F401
import math  # noqa: F401
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import toolz  # noqa: F401

SAMPLE_INPUT = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def count_arrangements_inner(
    cache: dict[tuple[tuple[str, ...], tuple[int, ...], bool], int],
    groups: list[str],
    numbers: list[int],
    first_group_started=False,
):
    cache_key = (tuple(groups), tuple(numbers), first_group_started)
    if cache_key in cache:
        return cache[cache_key]
    g = 0
    for i, number in enumerate(numbers):
        length_required = sum(numbers[i:]) + len(numbers[i:]) - 1
        length_remaining = sum(len(group) for group in groups[g:])
        if length_remaining < length_required:
            cache[cache_key] = 0
            return 0
        group_started = first_group_started
        first_group_started = False
        while number > 0:
            if g >= len(groups):
                cache[cache_key] = 0
                return 0
            if groups[g] == "":
                g += 1
            if g >= len(groups):
                cache[cache_key] = 0
                return 0
            match groups[g][0]:
                case ".":
                    if group_started:
                        cache[cache_key] = 0
                        return 0
                    g += 1
                case "#":
                    number -= len(groups[g])
                    group_started = True
                    g += 1
                case "?":
                    if not group_started:
                        ans = count_arrangements_inner(
                            cache, groups[g:], numbers[i:], True
                        ) + count_arrangements_inner(
                            cache, [groups[g][1:]] + groups[g + 1 :], numbers[i:]
                        )
                        cache[cache_key] = ans
                        return ans
                    else:
                        num_qs = len(groups[g])
                        if number >= num_qs:
                            number -= num_qs
                            g += 1
                        else:
                            groups[g] = "?" * (num_qs - number)
                            number = 0

        if number < 0:
            cache[cache_key] = 0
            return 0
        if g < len(groups):
            if "#" in groups[g]:
                cache[cache_key] = 0
                return 0
            groups[g] = groups[g][1:]

    if any("#" in group for group in groups[g:]):
        cache[cache_key] = 0
        return 0
    cache[cache_key] = 1
    return 1


def count_arrangements(springs: str, numbers: list[int]):
    groups = []
    group = springs[0]
    for c in springs[1:]:
        if c == group[0]:
            group += c
        else:
            groups.append(group)
            group = c
    groups.append(group)
    return count_arrangements_inner({}, groups, numbers)


def test_count_arrangements():
    assert count_arrangements("???.###", [1, 1, 3]) == 1
    assert count_arrangements(".??..??...?##.", [1, 1, 3]) == 4
    assert count_arrangements("?#?#?#?#?#?#?#?", [1, 3, 1, 6]) == 1
    assert count_arrangements("????.#...#...", [4, 1, 1]) == 1
    assert count_arrangements("????.######..#####.", [1, 6, 5]) == 4
    assert count_arrangements("?###????????", [3, 2, 1]) == 10


def part_a(input: str):
    total = 0
    for line in input.splitlines():
        springs, numbers = line.split()
        numbers = [int(n) for n in numbers.split(",")]
        total += count_arrangements(springs, numbers)
    return total


def part_b(input: str):
    total = 0
    for line in input.splitlines():
        springs, numbers = line.split()
        springs = "?".join(springs for _ in range(5))
        numbers = [int(n) for n in numbers.split(",")]
        numbers = numbers * 5
        total += count_arrangements(springs, numbers)
    return total


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 21
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 525152
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
