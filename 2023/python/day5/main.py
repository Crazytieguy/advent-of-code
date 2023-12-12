from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable, Generic, TypeVar

import toolz

SAMPLE_INPUT = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


@dataclass
class Mapping:
    from_range: range
    dest: int


T = TypeVar("T")


@dataclass
class MappingResult(Generic[T]):
    mapped: T | None = None
    not_mapped: list[T] = field(default_factory=list)


def part_a(input: str):
    seeds, all_mappings = parse_input(input)
    locations = map_all(seeds, all_mappings, map_single)
    return min(locations)


def part_b(input: str):
    seeds, all_mappings = parse_input(input)
    seed_ranges = [range(a, a + b) for a, b in toolz.itertoolz.partition(2, seeds)]
    location_ranges = map_all(seed_ranges, all_mappings, map_range)
    return min(r.start for r in location_ranges)


def map_all(
    objects: list[T],
    all_mappings: list[list[Mapping]],
    mapping_function: Callable[[Mapping, T], MappingResult[T]],
) -> list[T]:
    for mappings in all_mappings:
        next_objects = []
        for mapping in mappings:
            leftovers = []
            for o in objects:
                mapping_result = mapping_function(mapping, o)
                if mapping_result.mapped is not None:
                    next_objects.append(mapping_result.mapped)
                for not_mapped in mapping_result.not_mapped:
                    leftovers.append(not_mapped)
            objects = leftovers
        objects = next_objects + objects
    return objects


def map_single(mapping: Mapping, input: int) -> MappingResult[int]:
    if input in mapping.from_range:
        return MappingResult(mapped=input + mapping.dest - mapping.from_range.start)
    return MappingResult(not_mapped=[input])


def map_range(mapping: Mapping, input: range) -> MappingResult[range]:
    before = range(input.start, min(input.stop, mapping.from_range.start))

    mapped = range(
        max(input.start, mapping.from_range.start)
        + mapping.dest
        - mapping.from_range.start,
        min(input.stop, mapping.from_range.stop)
        + mapping.dest
        - mapping.from_range.start,
    )

    after = range(max(input.start, mapping.from_range.stop), input.stop)

    return MappingResult(
        mapped=mapped if not_empty(mapped) else None,
        not_mapped=list(filter(not_empty, [before, after])),
    )


def not_empty(r: range) -> bool:
    return r.stop > r.start


def parse_input(input: str) -> tuple[list[int], list[list[Mapping]]]:
    seeds, *all_mappings = input.split("\n\n")
    seeds = [int(seed) for seed in seeds.split()[1:]]
    all_mappings = [
        [parse_mapping(mapping) for mapping in mappings.splitlines()[1:]]
        for mappings in all_mappings
    ]
    return seeds, all_mappings


def parse_mapping(line: str) -> Mapping:
    dest, start, length = line.split()
    dest = int(dest)
    start = int(start)
    length = int(length)
    return Mapping(from_range=range(start, start + length), dest=dest)


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 35
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 46
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
