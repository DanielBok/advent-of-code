import re
from collections import defaultdict
from string import ascii_uppercase


def read_file():
    reg = re.compile(r'Step (\w) must be finished before step (\w) can begin\.')
    with open('d07.txt') as f:
        return [reg.findall(i)[0] for i in f.read().strip().split('\n')]


def q1():
    data = read_file()
    dd = defaultdict(set)

    for i, j in data:
        dd[i] = set()
        dd[j] = set()

    for i, j in data:
        dd[j].add(i)

    order = ''
    while len(dd) > 0:
        possible = []
        for k, v in dd.items():
            if len(v) == 0:
                possible.append(k)

        takeaway = sorted(possible)[0]
        order += takeaway
        for k, v in dd.items():
            dd[k] = v.difference(takeaway)

        dd.pop(takeaway)

    return order


def q2():
    data = read_file()
    dd = defaultdict(set)

    for i, j in data:
        dd[i] = set()
        dd[j] = set()

    for i, j in data:
        dd[j].add(i)

    workers = 5
    time = {j: i + 1 for i, j in enumerate(ascii_uppercase)}

    todo = []
    doing = []
    now = 0
    while len(dd) > 0 or len(doing) > 0 or len(todo) > 0:
        if len(doing) > 0:
            doing.sort(key=lambda x: x[1], reverse=True)
            out, t = doing.pop()
            now = t
            for k, v in dd.items():
                dd[k] = v.difference(out)

            workers += 1

        for k, v in dd.items():
            if len(v) == 0:
                todo.append(k)

        for k in todo:
            if k in dd.keys():
                dd.pop(k)

        todo.sort(reverse=True)
        while len(todo) > 0 and workers > 0:
            workers -= 1
            letter = todo.pop()
            doing.append((letter, now + 60 + time[letter]))

    return now
