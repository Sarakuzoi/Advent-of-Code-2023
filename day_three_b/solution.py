from typing import List

# It is important that we get rid of the right \n's, as they would be considered special characters
lines = [line.rstrip() for line in open("input.txt")]


# Goes to the right-most position that is still a digit, then finds the end of the number, returning it
def extract_num(row: int, col: int) -> int:
    if lines[row][col] == ".":
        return 1
    while col > 0 and lines[row][col].isdigit():
        col -= 1
    if not lines[row][col].isdigit():
        col += 1
    n = 0
    while col < len(lines[0]) and lines[row][col].isdigit():
        n = n * 10 + int(lines[row][col])
        col += 1
    return n


# Checks for 3 values on a row if they're part of a number
# Also returns how many values it found
def check_row(moves: List[int], row, col) -> List[int]:
    p, cnt = 1, 0
    for move in moves:
        if row + move[0] < 0 or col + move[1] < 0 or col + move[1] >= len(lines[0]):
            continue
        if lines[row + move[0]][col + move[1]].isdigit():
            p *= extract_num(row + move[0], col + move[1])
            cnt += 1
            if lines[row + move[0]][col] != ".":
                break
    return [p, cnt]


# Returns the number if it is a gear, 0 otherwise
def gearify(row: int, col: int) -> int:
    p, cnt = 1, 0
    moves_up = [(-1, -1), (-1, 0), (-1, 1)]
    moves_down = [(1, -1), (1, 0), (1, 1)]
    p *= check_row(moves_up, row, col)[0]
    cnt += check_row(moves_up, row, col)[1]
    p *= check_row(moves_down, row, col)[0]
    cnt += check_row(moves_down, row, col)[1]
    if col > 0:
        if extract_num(row, col - 1) != 1:
            cnt += 1
        p *= extract_num(row, col - 1)
    if col < len(lines[0]) - 1:
        if extract_num(row, col + 1) != 1:
            cnt += 1
        p *= extract_num(row, col + 1)
    return 0 if cnt != 2 else p


sum = 0
for i in range(len(lines)):
    for j in range(len(lines[0])):
        if lines[i][j] == "*":
            sum += gearify(i, j)

print(sum)
