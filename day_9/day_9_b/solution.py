input = open("input.txt").readlines()


class OasisHistory:
    def __init__(self, line) -> None:
        self.nums = []
        for v in line.split(" "):
            self.nums.append(int(v))


def calculate_value(arr) -> int:
    if all(v == 0 for v in arr):
        return 0
    n = len(arr)
    for i in range(1, n):
        arr.append(arr[i] - arr[i - 1])
    return arr[0] - calculate_value(arr[n:])


s = 0
for line in input:
    aux = OasisHistory(line)
    s += calculate_value(aux.nums)

print(s)
