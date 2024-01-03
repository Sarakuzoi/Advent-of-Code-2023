import heapq

input = open("input.txt").readlines()
m = {(i, j): int(c) for i, r in enumerate(input) for j, c in enumerate(r.strip())}
heat = {(0, 0): 0}
W = len(input[0].strip())
H = len(input)
vis = set()
queue = [(0, 0, 0, 0, 0, 0)]
while queue:
    curr, i, j, di, dj, dist = heapq.heappop(queue)
    if (i, j, di, dj, dist) in vis:
        continue
    vis.add((i, j, di, dj, dist))
    if i == H - 1 and j == W - 1:
        print(curr)
        break

    if dist < 3 and (di, dj) != (0, 0) and 0 <= i + di < H and 0 <= j + dj < W:
        heapq.heappush(
            queue, (curr + m[(i + di, j + dj)], i + di, j + dj, di, dj, dist + 1)
        )
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
