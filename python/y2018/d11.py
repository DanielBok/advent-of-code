from typing import Tuple

import numpy as np
from concurrent.futures import ProcessPoolExecutor

N = 300


def make_grid(gsn: int):
    rack_id = np.repeat(np.arange(N), N).reshape(N, N) + 11
    grid = np.copy(rack_id)
    grid *= np.arange(1, N + 1)
    grid += gsn
    grid *= rack_id
    grid = np.array((np.abs(grid) > 100) * grid / 100 % 10, dtype=int) - 5

    return grid


def test_grid():
    for g, e, x, y in [(57, -5, 122, 79),
                       (39, 0, 217, 196),
                       (71, 4, 101, 153)]:
        grid = make_grid(g)
        assert grid[x - 1, y - 1] == e

    for g, x, y in [(18, 33, 45),
                    [42, 21, 61]]:
        assert (x, y) == q1(g)


def get_max(grid: np.array) -> Tuple[int, int, int]:
    _max = int(grid.max())
    x, y = np.where(grid == _max)
    x = x[0] + 1
    y = y[0] + 1
    return _max, x, y


def q1(gsn: int):
    grid = make_grid(gsn)
    grid = convolve(grid, 3)

    _, x, y = get_max(grid)
    return x, y


def convolve(grid: np.array, size: int):
    kernel = np.ones(size ** 2).reshape(size, size)

    s = N - size + 1
    _grid = np.zeros(s ** 2).reshape(s, s)

    for i in range(s):
        for j in range(s):
            _grid[i, j] = np.sum(kernel * grid[i:i + size, j:j + size])

    return _grid


def _q2(grid, i):
    return (*get_max(convolve(grid, i)), i)


def q2(gsn: int):
    grid = make_grid(gsn)
    _max, x, y = get_max(grid)

    results = [(_max, x, y, 1)]

    with ProcessPoolExecutor(3) as P:
        calls = list(range(2, N + 1))
        _res = P.map(_q2, (grid for _ in calls), calls)

    results.append(list(_res))

    results = np.array(results)
    i = results.argmax(0)[0]

    _, x, y, size = results[i]

    return x, y, size


if __name__ == '__main__':
    GSN = 5791
    print(q1(GSN))
    print(q2(GSN))
