from collections import defaultdict

cards_input = open("input.txt").readlines()


class Card:
    def __init__(self, line):
        words = line.split()
        self.id = int(words[1][:-1])
        winning, numbers = set(), set()
        is_num = False
        for i in range(2, len(words)):
            if words[i] == "|":
                is_num = True
                continue
            if is_num:
                numbers.add(int(words[i]))
            else:
                winning.add(int(words[i]))
        self.winning = winning
        self.numbers = numbers

    def win(self) -> int:
        champs = 0
        for n in self.numbers:
            if n in self.winning:
                champs += 1
        return champs


card_map = defaultdict(int)
cards = []
for card_line in cards_input:
    cards.append(Card(card_line))
    card_map[cards[-1].id] += 1
    sum = cards[-1].win()
    for i in range(cards[-1].id + 1, cards[-1].id + sum + 1):
        card_map[i] += card_map[cards[-1].id]

res = 0
for v in card_map.values():
    res += v
print(res)
