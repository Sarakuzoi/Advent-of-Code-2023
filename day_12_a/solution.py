contents = open("input.txt").readlines()


def valid(line: str, groups: [int]) -> bool:
    size, ind = 0, 0
    for c in line:
        if c == "#":
            size += 1
        elif size != 0:
            if ind >= len(groups) or groups[ind] != size:
                return False
            size = 0
            ind += 1
    return ind == len(groups)


def generate(springs, groups, ind) -> int:
    if "?" not in springs:
        return 1 if valid(springs, groups) else 0
    for i in range(ind, len(springs)):
        if springs[i] == "?":
            ind = i
            break
    cnt = 0
    springs = springs[:ind] + "." + springs[ind + 1 :]
    cnt += generate(springs, groups, ind)
    springs = springs[:ind] + "#" + springs[ind + 1 :]
    cnt += generate(springs, groups, ind)
    return cnt


def count_possibilities(line):
    line = line.strip()
    parts = line.split(" ")
    springs = "." + parts[0] + "."
    groups = []
    for n in parts[1].split(","):
        groups.append(int(n))
    return generate(springs, groups, 0)


res = 0
for line in contents:
    res += count_possibilities(line)

print(res)
