from typing import List

# It is important that we get rid of the right \n's, as they would be considered special characters
lines = [line.rstrip() for line in open("input.txt")]


# Returns the number if it is a gear, 0 otherwise
def validate(num_columns: List[int], row: int) -> int:
    res = int(lines[row][num_columns[0] : num_columns[-1] + 1])
    num_columns.insert(0, num_columns[0] - 1)
    num_columns.append(num_columns[-1] + 1)
    for c in num_columns:
        if c < 0 or c >= len(lines[0]):
            continue
        if row > 0 and lines[row - 1][c] != ".":
            return res
        if row < len(lines) - 1 and lines[row + 1][c] != ".":
            return res
    if num_columns[1] > 0 and lines[row][num_columns[1] - 1] != ".":
        return res
    if num_columns[-2] < len(lines[0]) - 1 and lines[row][num_columns[-1]] != ".":
        return res
    return 0


sum = 0
for i in range(len(lines)):
    # We use aux to store the position of the digits in a line (an interval would've worked fine as well)
    aux = []
    for j in range(len(lines[0])):
        if lines[i][j].isdigit():
            aux.append(j)
        elif aux:
            sum += validate(aux, i)
            aux = []
    if aux:
        sum += validate(aux, i)

print(sum)
