import heapq

input = open("input.txt").readlines()
m = {(i, j): int(c) for i, r in enumerate(input) for j, c in enumerate(r.strip())}
W = len(input[0].strip())
H = len(input)
vis = set()
queue = [(0, 0, 0, 1, 0, 0), (0, 0, 0, 0, 1, 0)]
res = 1e20

while queue:
    curr, i, j, di, dj, dist = heapq.heappop(queue)
    if (i, j, di, dj, dist) in vis:
        continue
    vis.add((i, j, di, dj, dist))

    outOfBounds = False
    while dist < 4:
        i += di
        j += dj
        dist += 1
        if not (0 <= i < H and 0 <= j < W):
            outOfBounds = True
            break
        curr += m[(i, j)]
    if outOfBounds:
        continue

    while dist <= 10 and 0 <= i < H and 0 <= j < W:
        if i == H - 1 and j == W - 1:
            res = min(res, curr)
            break
        for ni, nj in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            if (
                (ni, nj) != (di, dj)
                and (ni, nj) != (-di, -dj)
                and 0 <= i + ni < H
                and 0 <= j + nj < W
            ):
                heapq.heappush(
                    queue, (curr + m[(i + ni, j + nj)], i + ni, j + nj, ni, nj, 1)
                )
        i += di
        j += dj
        dist += 1
        if not (0 <= i < H and 0 <= j < W):
            break
        curr += m[(i, j)]

print(res)
