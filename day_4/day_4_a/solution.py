cards_input = open("input.txt").readlines()


class Card:
    def __init__(self, id, winning, numbers):
        self.id = id
        self.winning = winning
        self.numbers = numbers

    def __init__(self, line):
        words = line.split()
        self.id = words[1][:-1]
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
        champs = 1
        for n in self.numbers:
            if n in self.winning:
                champs *= 2
        return champs // 2


cards = []
for card_line in cards_input:
    cards.append(Card(card_line))

sum = 0
for card in cards:
    sum += card.win()

print(sum)
