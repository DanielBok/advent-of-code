from typing import Dict, Optional, Tuple


Coords = Tuple[int, int]
Locs = Dict[Coords, Tuple[str, int]]


def read_data(file='d13.txt'):
    with open(file) as f:
        raw = f.read()

    raw_map = raw.replace('>', '-').replace('<', '-').replace('^', '|').replace('v', '|').split('\n')

    max_len = max(len(i) for i in raw_map)
    graph = [list(j) for j in [i + ' ' * (max_len - len(i)) for i in raw_map]]

    locs: dict = {}

    for y, line in enumerate(raw.split('\n')):
        for x, letter in enumerate(line):
            if letter in {'v', '^', '<', '>'}:
                locs[(x, y)] = (letter, 0)

    return graph, locs


def move_point(graph, x, y, d, t) -> Locs:
    x, y, d, t = _move_point(graph, x, y, d, t)
    return {(x, y): (d, t)}


def _move_point(graph, x, y, d, t):
    road = graph[y][x]
    if d == '^':
        if road == '\\':
            d = '<'
        elif road == '/':
            d = '>'
        elif road == '+':
            if t % 3 == 0:
                d = '<'
            elif t % 3 == 2:
                d = '>'
            t += 1
        elif road == '-':
            raise ValueError(f'road cannot be "-" when d is "{d}"')

    elif d == 'v':
        if road == '\\':
            d = '>'
        elif road == '/':
            d = '<'
        elif road == '+':
            if t % 3 == 0:
                d = '>'
            elif t % 3 == 2:
                d = '<'
            t += 1
        elif road == '-':
            raise ValueError(f'road cannot be "-" when d is "{d}"')

    elif d == '>':
        if road == '\\':
            d = 'v'
        elif road == '/':
            d = '^'
        elif road == '+':
            if t % 3 == 0:
                d = '^'
            elif t % 3 == 2:
                d = 'v'
            t += 1
        elif road == '|':
            raise ValueError(f'road cannot be "|" when d is "{d}"')

    elif d == '<':
        if road == '\\':
            d = '^'
        elif road == '/':
            d = 'v'
        elif road == '+':
            if t % 3 == 0:
                d = 'v'
            elif t % 3 == 2:
                d = '^'
            t += 1
        elif road == '|':
            raise ValueError(f'road cannot be "|" when d is "{d}"')

    return x, y, d, t


def move1(graph: list, locs: Locs) -> Tuple[Locs, Optional[Coords]]:
    new_loc = {}
    points = set(locs.keys())

    for (x, y), (d, t) in locs.items():
        points.remove((x, y))
        if d == '^':
            y -= 1
        if d == 'v':
            y += 1
        if d == '>':
            x += 1
        if d == '<':
            x -= 1

        if (x, y) in points:
            return locs, (x, y)
        elif new_loc.get((x, y), None) is not None:
            return new_loc, (x, y)

        new_loc.update(move_point(graph, x, y, d, t))

    return new_loc, None


def move2(graph: list, locs: Locs, s: int) -> Locs:
    new_loc = {}
    points = set(k for k in locs.keys())
    has_crashed = {i: False for i in points}

    order = sorted(points)

    for (x, y) in order:
        o_coord = (x, y)
        d, t = locs[o_coord]

        if has_crashed[o_coord]:
            continue

        if d == '^':
            y -= 1
        elif d == 'v':
            y += 1
        elif d == '>':
            x += 1
        elif d == '<':
            x -= 1

        if (x, y) in points:
            has_crashed[(x, y)] = True
            points.remove((x, y))
            print(f'Crash at {x}, {y}. T = {s}')
        elif new_loc.get((x, y), None) is not None:
            print(f'Crash at {x}, {y}. T = {s}')
            new_loc.pop((x, y))
        else:
            new_loc.update(move_point(graph, x, y, d, t))
            points.remove(o_coord)

    return new_loc


def q1():
    graph, locs = read_data()

    while True:
        locs, collision = move1(graph, locs)
        if collision is not None:
            return collision


def q2():
    graph, locs = read_data()

    s = 0
    while True:
        locs = move2(graph, locs, s)
        if len(locs) == 1:
            for k in locs.keys():
                return k
        if len(locs) == 0:
            return
        s += 1


print(q1())
print(q2())
