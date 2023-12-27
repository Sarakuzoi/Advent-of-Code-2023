contents = open("input.txt").readlines()
from functools import cache


@cache
def generate(springs, sizes, group_size) -> int:
    if not springs:
        return not sizes and not group_size
    cnt = 0
    curr = [".", "#"] if springs[0] == "?" else springs[0]
    for c in curr:
        if c == "#":
            group_size += 1
            cnt += generate(springs[1:], sizes, group_size)
        elif group_size != 0:
            if sizes and sizes[0] == group_size:
                cnt += generate(springs[1:], sizes[1:], 0)
        else:
            cnt += generate(springs[1:], sizes, 0)
    return cnt


def count_possibilities(line):
    line = line.strip()
    parts = line.split(" ")
    springs = (
        "."
        + parts[0]
        + "?"
        + parts[0]
        + "?"
        + parts[0]
        + "?"
        + parts[0]
        + "?"
        + parts[0]
        + "."
    )
    groups, copy = [], []
    for n in parts[1].split(","):
        groups.append(int(n))
        copy.append(int(n))
    for _ in range(4):
        groups.extend(copy)

    return generate(tuple(springs), tuple(groups), 0)


res = 0
for line in contents:
    res += count_possibilities(line)

print(res)
