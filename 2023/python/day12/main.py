from pathlib import Path

SAMPLE_INPUT = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def part_a(input: str):
    total = 0
    for line in input.splitlines():
        springs, group_sizes = line.split()
        group_sizes = [int(n) for n in group_sizes.split(",")]
        total += count_arrangements(springs, group_sizes)
    return total


def part_b(input: str):
    total = 0
    for line in input.splitlines():
        springs, group_sizes = line.split()
        group_sizes = [int(n) for n in group_sizes.split(",")]
        springs = "?".join(springs for _ in range(5))
        group_sizes = group_sizes * 5
        total += count_arrangements(springs, group_sizes)
    return total


def count_arrangements(
    springs: str, group_sizes: list[int], cache: list[list[int | None]] | None = None
) -> int:
    if cache is None:
        # e for end, to cover an edge case
        springs += "e"
        cache = [[None for _ in range(len(group_sizes))] for _ in range(len(springs))]

    if not group_sizes:
        if "#" in springs:
            # too many ?s were replaced with #
            return 0
        # all remaining ?s are .
        return 1

    if len(springs) < sum(group_sizes) + len(group_sizes):
        # not enough space for remaining numbers
        return 0

    if (cached := cache[len(springs) - 1][len(group_sizes) - 1]) is not None:
        return cached

    arangements = 0
    if springs[0] in ".?":
        arangements += count_arrangements(springs[1:], group_sizes, cache)

    next_group_size = group_sizes[0]
    if "." not in springs[:next_group_size] and springs[next_group_size] != "#":
        skip = next_group_size + 1
        arangements += count_arrangements(springs[skip:], group_sizes[1:], cache)

    cache[len(springs) - 1][len(group_sizes) - 1] = arangements
    return arangements


def test_count_arrangements():
    assert count_arrangements("???.###", [1, 1, 3]) == 1
    assert count_arrangements(".??..??...?##.", [1, 1, 3]) == 4
    assert count_arrangements("?#?#?#?#?#?#?#?", [1, 3, 1, 6]) == 1
    assert count_arrangements("????.#...#...", [4, 1, 1]) == 1
    assert count_arrangements("????.######..#####.", [1, 6, 5]) == 4
    assert count_arrangements("?###????????", [3, 2, 1]) == 10


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 21
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 525152
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
