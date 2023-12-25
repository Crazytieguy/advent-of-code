import collections  # noqa: F401
import copy
import itertools  # noqa: F401
import math  # noqa: F401
import random
import re  # noqa: F401
from dataclasses import dataclass  # noqa: F401
from pathlib import Path

import toolz  # noqa: F401

SAMPLE_INPUT = """jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"""
INPUT = Path("data.txt").read_text()


def connected_components(graph: dict[str, set[str]]) -> set[int]:
    seen = set()
    components = set()
    for node in graph:
        if node not in seen:
            component = set()
            stack = [node]
            while stack:
                node = stack.pop()
                if node not in component:
                    component.add(node)
                    seen.add(node)
                    stack.extend(graph[node])
            components.add(len(component))
    return components


def part_a(input: str):
    graph = collections.defaultdict(set)
    for line in input.splitlines():
        (source, destinations) = line.split(": ")
        for destination in destinations.split():
            graph[source].add(destination)
            graph[destination].add(source)
    graph = dict(graph)
    while True:
        seen = set()
        test_graph = copy.deepcopy(graph)
        for node, adjacent in graph.items():
            viable_remove_choices = [a for a in adjacent if a not in seen]
            if node in seen or not viable_remove_choices:
                continue
            remove = random.choice(viable_remove_choices)
            test_graph[node].remove(remove)
            test_graph[remove].remove(node)
            seen.add(node)
            seen.add(remove)
        components = connected_components(test_graph)
        if len(components) == 2:
            first, second = components
            return first * second


def test_part_a(capsys):
    with capsys.disabled():
        assert part_a(SAMPLE_INPUT) == 54
        print(f"Part A: {part_a(INPUT)}")
