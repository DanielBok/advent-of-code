import numpy as np


def read_data(file='d25'):
    if not file.endswith('.txt'):
        file += '.txt'
    with open(file) as f:
        return np.array([[int(i) for i in line.strip().split(',')] for line in f])


def test_part1():
    assert part1('d25a') == 4
    assert part1('d25b') == 3
    assert part1('d25c') == 8
    assert part1('d25d') == 2


def part1(file='d25'):
    data = read_data(file)
    initial = []

    for cs in data:
        close = set([tuple(i) for i in data[np.abs(data - cs).sum(1) <= 3]])

        for cset in initial:
            if cset & close:
                cset |= close
                break
        else:
            initial.append(close)

    while True:
        const = set()
        for cs1 in initial:
            for cs2 in initial:
                if cs1 & cs2:
                    cs1 |= cs2

            const.add(tuple(sorted(cs1)))

        if len(const) == len(initial):
            break
        initial = [set(i) for i in const]

    return len(const)


test_part1()
part1()