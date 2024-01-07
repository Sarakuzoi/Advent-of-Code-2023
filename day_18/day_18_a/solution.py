from collections import defaultdict


input = open("input.txt").readlines()
margins = [(line.split(" ")[0], int(line.split(" ")[1])) for line in input]
lagoon = defaultdict(lambda _: ".")

ci, cj = 0, 0
maxi, maxj = 0, 0
mini, minj = float("inf"), float("inf")
for d, l in margins:
    match d:
        case "R":
            for _ in range(l - 1):
                cj += 1
                lagoon[(ci, cj)] = "R"
            cj += 1
            maxj = max(maxj, cj)
        case "L":
            for _ in range(l - 1):
                cj -= 1
                lagoon[(ci, cj)] = "L"
            cj -= 1
            minj = min(minj, cj)
        case "U":
            for _ in range(l):
                lagoon[(ci, cj)] = "U"
                ci -= 1
            lagoon[(ci, cj)] = "U"
            mini = min(mini, ci)
        case "D":
            for _ in range(l):
                lagoon[(ci, cj)] = "D"
                ci += 1
            lagoon[(ci, cj)] = "D"
            maxi = max(maxi, ci)

num = 0
mostRecent = ""
for i in range(mini, maxi + 2):
    for j in range(minj, maxj + 2):
        if (i, j) in lagoon:
            if lagoon[(i, j)] == "U" and mostRecent != "U":
                num += 1
                mostRecent = "U"
            if lagoon[(i, j)] == "D" and mostRecent != "D":
                num -= 1
                mostRecent = "D"
        if num != 0:
            lagoon[(i, j)] = "#"
print(len(lagoon))
