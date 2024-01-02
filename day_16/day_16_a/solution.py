from enum import Enum, auto
import sys
from functools import cache

sys.setrecursionlimit(100000)


class Direction(Enum):
    UP = auto()
    DOWN = auto()
    LEFT = auto()
    RIGHT = auto()

    @staticmethod
    def getDirection(i, j, previ, prevj):
        if i == previ and j > prevj:
            return Direction.RIGHT
        if i == previ and j < prevj:
            return Direction.LEFT
        if j == prevj and i > previ:
            return Direction.DOWN
        if j == prevj and i < previ:
            return Direction.UP


with open("input.txt") as input:
    matrix = [list(line.strip()) for line in input.readlines()]

vis = set()
aux = set()


stack = [(0, 0, 0, -1)]
while stack:
    i, j, previ, prevj = stack.pop()
    if i < 0 or i >= len(matrix) or j < 0 or j >= len(matrix[0]):
        continue
    if (i, j, previ, prevj) in aux:
        continue
    aux.add((i, j, previ, prevj))
    vis.add((i, j))
    dir = Direction.getDirection(i, j, previ, prevj)
    match matrix[i][j]:
        case ".":
            match dir:
                case Direction.UP:
                    stack.append((i - 1, j, i, j))
                case Direction.DOWN:
                    stack.append((i + 1, j, i, j))
                case Direction.LEFT:
                    stack.append((i, j - 1, i, j))
                case Direction.RIGHT:
                    stack.append((i, j + 1, i, j))
        case "-":
            match dir:
                case Direction.UP | Direction.DOWN:
                    stack.append((i, j + 1, i, j))
                    stack.append((i, j - 1, i, j))
                case Direction.LEFT:
                    stack.append((i, j - 1, i, j))
                case Direction.RIGHT:
                    stack.append((i, j + 1, i, j))
        case "|":
            match dir:
                case Direction.LEFT | Direction.RIGHT:
                    stack.append((i - 1, j, i, j))
                    stack.append((i + 1, j, i, j))
                case Direction.UP:
                    stack.append((i - 1, j, i, j))
                case Direction.DOWN:
                    stack.append((i + 1, j, i, j))
        case "/":
            match dir:
                case Direction.UP:
                    stack.append((i, j + 1, i, j))
                case Direction.DOWN:
                    stack.append((i, j - 1, i, j))
                case Direction.LEFT:
                    stack.append((i + 1, j, i, j))
                case Direction.RIGHT:
                    stack.append((i - 1, j, i, j))
        case "\\":
            match dir:
                case Direction.UP:
                    stack.append((i, j - 1, i, j))
                case Direction.DOWN:
                    stack.append((i, j + 1, i, j))
                case Direction.LEFT:
                    stack.append((i - 1, j, i, j))
                case Direction.RIGHT:
                    stack.append((i + 1, j, i, j))


print(len(vis))
