import re
from dataclasses import dataclass
from pathlib import Path

SAMPLE_INPUT = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""
INPUT = Path("data.txt").read_text()


@dataclass
class Game:
    red: int
    green: int
    blue: int


def part_a(input: str):
    games = [game(line) for line in input.splitlines()]
    return sum(
        i + 1
        for i, game in enumerate(games)
        if game.red <= 12 and game.green <= 13 and game.blue <= 14
    )


def part_b(input: str):
    games = [game(line) for line in input.splitlines()]
    return sum(game.red * game.green * game.blue for game in games)


RED = re.compile(r"(\d+) red")
GREEN = re.compile(r"(\d+) green")
BLUE = re.compile(r"(\d+) blue")


def game(line: str):
    red = max(int(match.group(1)) for match in RED.finditer(line))
    green = max(int(match.group(1)) for match in GREEN.finditer(line))    
    blue = max(int(match.group(1)) for match in BLUE.finditer(line))

    return Game(
        red=red,
        green=green,
        blue=blue,
    )


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 8
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT) == 2286
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
