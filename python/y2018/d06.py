from typing import List, Tuple


def read_data() -> Tuple[List[int], List[int]]:
    with open('d06.txt') as f:
        _data = f.read().strip().split('\n')
        _xs, _ys = [], []
        for line in _data:
            x, y = [int(i) for i in line.split(',')]
            _xs.append(x)
            _ys.append(y)

    return _xs, _ys


def q1():
    xs, ys = read_data()
    min_x, max_x = 0, max(xs) + 1
    min_y, max_y = 0, max(ys) + 1

    range_x = list(range(max_x - min_x))
    range_y = list(range(max_y - min_y))

    area = [[0 for _ in range_x] for _ in range_y]
    points = [(i + 1, t) for i, t in list(enumerate(zip(xs, ys)))]

    for i, (x, y) in points:
        area[y][x] = i

    for i in range_y:
        for j in range_x:
            if area[i][j] != 0:
                continue

            min_dist = max_x + max_y
            min_point = 0
            for p, (x, y) in points:
                dist = abs(x - j) + abs(y - i)
                if dist < min_dist:
                    min_dist = dist
                    min_point = p
                elif dist == min_dist:
                    min_point = 0

            area[i][j] = min_point

    edge = {*area[0], *area[-1]}
    for row in area[1: -1]:
        edge.add(row[0])
        edge.add(row[-1])

    max_count = 0
    for p, _ in points:
        if p in edge:
            continue

        count = 0
        for row in area:
            for e in row:
                if e == p:
                    count += 1

        if count > max_count:
            max_count = count

    return max_count


def q2():
    xs, ys = read_data()
    min_x, max_x = 0, max(xs) + 1
    min_y, max_y = 0, max(ys) + 1

    range_x = list(range(max_x - min_x))
    range_y = list(range(max_y - min_y))

    area = [[0 for _ in range_x] for _ in range_y]
    points = [(i + 1, t) for i, t in list(enumerate(zip(xs, ys)))]

    for i in range_y:
        for j in range_x:
            total_dist = 0
            area[i][j] = 1
            for p, (x, y) in points:
                total_dist += abs(x - j) + abs(y - i)
                if total_dist > 10000:
                    area[i][j] = 0
                    break

    return sum(sum(row) for row in area)
