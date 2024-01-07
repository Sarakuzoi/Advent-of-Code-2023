dirs = {
    "R": (1, 0),
    "D": (0, 1),
    "L": (-1, 0),
    "U": (0, -1),
}


def convert(line: str):
    aux = line.split(" ")[2]
    word = aux[2 : len(aux) - 1]
    direction = ""
    match word[-1]:
        case "0":
            direction = "R"
        case "1":
            direction = "D"
        case "2":
            direction = "L"
        case "3":
            direction = "U"
    length = int(word[:-1], 16)
    return (direction, length)


input = list(map(lambda x: x.strip(), open("input.txt").readlines()))
steps = [(dirs[convert(line)[0]], convert(line)[1]) for line in input]


def count(steps):
    pos, ans = 0, 1
    for (x, y), n in steps:
        pos += x * n
        ans += y * n * pos + n / 2

    return int(ans)


print(count(steps))
