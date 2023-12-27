import sys


contents = open("input.txt").readlines()


def checkUp(i, j) -> bool:
    if i == 0:
        return False
    pos, up = contents[i][j], contents[i - 1][j]

    if pos != "|" and pos != "L" and pos != "J" and pos != "S":
        return False
    if up == "S":
        return True
    if up != "|" and up != "F" and up != "7":
        return False
    return True


def checkDown(i, j) -> bool:
    if i == len(contents) - 1:
        return False
    pos, down = contents[i][j], contents[i + 1][j]

    if pos != "|" and pos != "F" and pos != "7" and pos != "S":
        return False
    if down == "S":
        return True
    if down != "|" and down != "J" and down != "L":
        return False
    return True


def checkRight(i, j) -> bool:
    if j == len(contents[0]) - 1:
        return False
    pos, right = contents[i][j], contents[i][j + 1]

    if pos != "-" and pos != "F" and pos != "L" and pos != "S":
        return False
    if right == "S":
        return True
    if right != "-" and right != "J" and right != "7":
        return False
    return True


def checkLeft(i, j) -> bool:
    if j == 0:
        return False
    pos, left = contents[i][j], contents[i][j - 1]

    if pos != "-" and pos != "7" and pos != "J" and pos != "S":
        return False
    if left == "S":
        return True
    if left != "-" and left != "F" and left != "L":
        return False
    return True


start = [0, 0]
vis = set()
for i in range(len(contents)):
    contents[i] = contents[i].strip()
    if contents[i].find("S") != -1:
        start = (i, contents[i].find("S"))
        break

# Python has a maximum recursion depth of 1000 :,)
sys.setrecursionlimit(len(contents) * len(contents[0]))


def search(i, j, cnt) -> int:
    if tuple(start) in vis and cnt > 1:
        vis.remove(tuple(start))
    if contents[i][j] == "S" and cnt != 0:
        return cnt
    if checkUp(i, j) and (i - 1, j) not in vis:
        vis.add((i, j))
        aux = search(i - 1, j, cnt + 1)
        if aux != -1:
            return aux
        vis.remove((i, j))
    if checkDown(i, j) and (i + 1, j) not in vis:
        vis.add((i, j))
        aux = search(i + 1, j, cnt + 1)
        if aux != -1:
            return aux
        vis.remove((i, j))
    if checkLeft(i, j) and (i, j - 1) not in vis:
        vis.add((i, j))
        aux = search(i, j - 1, cnt + 1)
        if aux != -1:
            return aux
        vis.remove((i, j))
    if checkRight(i, j) and (i, j + 1) not in vis:
        vis.add((i, j))
        aux = search(i, j + 1, cnt + 1)
        if aux != -1:
            return aux
        vis.remove((i, j))
    return -1


res = search(start[0], start[1], 0)

print(res // 2)
