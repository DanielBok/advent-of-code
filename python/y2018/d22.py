import numpy as np
from collections import deque


def form_grids(depth, target, extension=0):
    mod = 20183
    grid_x, grid_y = np.array(target) + 1 + extension

    egrid = np.zeros((grid_x, grid_y), dtype=np.uint64)

    egrid[0, :] = np.arange(grid_y) * 48271 + depth
    egrid[:, 0] = np.arange(grid_x) * 16807 + depth
    egrid %= mod

    egrid[0, 0] = 0

    for i in range(1, grid_x):
        for j in range(1, grid_y):
            if (i, j) == target or (i, j) == (0, 0):
                egrid[i, j] = 0
            else:
                egrid[i, j] = ((egrid[i - 1, j] * egrid[i, j - 1]) + depth) % mod

    sgrid = egrid % 3
    return egrid, sgrid


def part1(depth=11541, target=(14, 778)):
    _, sgrid = form_grids(depth, target)
    x, y = target
    return sgrid[:x + 1, :y + 1].sum()


def test_part1():
    depth = 510
    target = 10, 10

    assert part1(depth, target) == 114


def part2(depth=11541, target=(14, 778), ext=50, return_paths=False, hard_num=float('inf')):
    _map = {0: 'R', 1: 'W', 2: 'N'}
    _, grid = form_grids(depth, target, ext)
    grid = np.array([[_map[e] for e in row] for row in grid])
    xmax, ymax = grid.shape

    start = (0, 0, 'T')
    queue = deque([])
    queue.append(start)
    dist = {start: 0}
    target_dist = np.inf

    paths = {}

    def append(_i, _j, _d, _x, _y, _ceq, _eq):
        key = _i, _j, _eq
        if key not in dist or dist[key] > _d:
            dist[key] = _d
            queue.append(key)
            paths[key] = _x, _y, _ceq, _i, _j, _eq, _d

    while len(queue) > 0:
        x, y, eq = queue.popleft()
        d = dist[(x, y, eq)]
        cell = grid[x, y]
        if d > target_dist or d > hard_num:
            continue

        n, s, e, w = (x, y - 1), (x, y + 1), (x + 1, y), (x - 1, y)
        for i, j in (n, s, e, w):
            if i < 0 or i >= xmax or j < 0 or j >= ymax:
                continue

            # ending condition
            if (i, j) == target:
                total_dist = d + 1 if eq == 'T' else d + 8
                if total_dist < target_dist:
                    target_dist = total_dist
                    dist[(i, j, 'T')] = total_dist
                    paths[(i, j, 'T')] = x, y, eq, i, j, 'T', total_dist
                continue

            next_cell = grid[i, j]
            # normal exploration condition
            if cell == next_cell:
                append(i, j, d + 1, x, y, eq, eq)

            elif eq == 'O' and next_cell in {'W', 'N'}:
                append(i, j, d + 1, x, y, eq, eq)

            elif eq == 'T' and next_cell in {'R', 'N'}:
                append(i, j, d + 1, x, y, eq, eq)

            elif eq == 'C' and next_cell in {'W', 'R'}:
                append(i, j, d + 1, x, y, eq, eq)

            elif cell == 'R' and next_cell == 'W':
                append(i, j, d + 8, x, y, eq, 'C')

            elif cell == 'R' and next_cell == 'N':
                append(i, j, d + 8, x, y, eq, 'T')

            elif cell == 'W' and next_cell == 'R':
                append(i, j, d + 8, x, y, eq, 'C')

            elif cell == 'W' and next_cell == 'N':
                append(i, j, d + 8, x, y, eq, 'O')

            elif cell == 'N' and next_cell == 'R':
                append(i, j, d + 8, x, y, eq, 'T')

            elif cell == 'N' and next_cell == 'W':
                append(i, j, d + 8, x, y, eq, 'O')

    nodes = [(*target, 'T', target_dist)]
    while nodes[-1][:3] != start:
        nodes.append(paths[nodes[-1][:3]])

    if return_paths:
        return target_dist, nodes
    else:
        return target_dist


def test_part2():
    assert part2(510, (10, 10), 10) == 45
    n = part2(hard_num=1100)
    print(n)
    assert n == 1068


# test_part1()
# print(part1())


ans, paths = part2(return_paths=True)
