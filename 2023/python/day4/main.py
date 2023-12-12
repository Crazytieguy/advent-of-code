from pathlib import Path

SAMPLE_INPUT = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"""
SAMPLE_INPUT_B = SAMPLE_INPUT
INPUT = Path("data.txt").read_text()


def part_a(input: str):
    return sum(card_score(*parse_card(card)) for card in input.splitlines())


def part_b(input: str):
    card_scores = [card_wins(*parse_card(card)) for card in input.splitlines()]
    card_copies = [1] * len(card_scores)
    for i, score in enumerate(card_scores):
        for j in range(i + 1, score + i + 1):
            try:
                card_copies[j] += card_copies[i]
            except IndexError:
                break
    return sum(card_copies)


def parse_card(card: str) -> tuple[set[str], set[str]]:
    _, numbers = card.split(":")
    winning, owned = numbers.split("|")
    winning = set(winning.split())
    owned = set(owned.split())
    return winning, owned


def card_score(winning: set[str], owned: set[str]) -> int:
    wins = card_wins(winning, owned)
    return 2**wins // 2


def card_wins(winning: set[str], owned: set[str]) -> int:
    return len(winning & owned)


def test_part_a(capsys):
    assert part_a(SAMPLE_INPUT) == 13
    with capsys.disabled():
        print(f"Part A: {part_a(INPUT)}")


def test_part_b(capsys):
    assert part_b(SAMPLE_INPUT_B) == 30
    with capsys.disabled():
        print(f"Part B: {part_b(INPUT)}")
