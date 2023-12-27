input = open("input.txt").readlines()


class OasisHistory:
    def __init__(self, line) -> None:
        self.nums = []
        for v in line.split(" "):
            self.nums.append(int(v))

    def calculate_value(self) -> int:
        s = 0
        while not all(v == 0 for v in self.nums):
            n = len(self.nums)
            s += self.nums[-1]
            for i in range(1, n):
                self.nums.append(self.nums[i] - self.nums[i - 1])
            self.nums = self.nums[n:]
        return s


s = 0
for line in input:
    aux = OasisHistory(line)
    s += aux.calculate_value()

print(s)
