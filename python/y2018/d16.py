import re

from typing import List, Tuple, Dict

Arr = Tuple[int, int, int, int]
Data = List[Tuple[Arr, Arr, Arr]]


def read_op_code_data():
    with open('d16oc.txt') as f:
        raw = f.read().strip().split('\n\n')

    reg = re.compile(r'(\d+),? (\d+),? (\d+),? (\d+)')

    data: Data = []
    for line in raw:
        before, cmd, after = [tuple(int(i) for i in arr) for arr in reg.findall(line)]
        data.append((before, cmd, after))

    return data


def read_test_code():
    with open('d16tc.txt') as f:
        return [tuple(int(i) for i in line.strip().split()) for line in f.read().strip().split('\n')]


def addr(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] + data[b]
    return tuple(ndata)


def addi(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] + b
    return tuple(ndata)


def mulr(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] * data[b]
    return tuple(ndata)


def muli(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] * b
    return tuple(ndata)


def banr(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] & data[b]
    return tuple(ndata)


def bani(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] & b
    return tuple(ndata)


def borr(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] | data[b]
    return tuple(ndata)


def bori(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a] | b
    return tuple(ndata)


def setr(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = data[a]
    return tuple(ndata)


def seti(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = a
    return tuple(ndata)


def gtir(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = 1 if a > data[b] else 0
    return tuple(ndata)


def gtri(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = 1 if data[a] > b else 0
    return tuple(ndata)


def gtrr(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = 1 if data[a] > data[b] else 0
    return tuple(ndata)


def eqir(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = 1 if a == data[b] else 0
    return tuple(ndata)


def eqri(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = 1 if data[a] == b else 0
    return tuple(ndata)


def eqrr(cmd: Arr, data: Arr):
    op, a, b, c = cmd
    ndata = list(data)
    ndata[c] = 1 if data[a] == data[b] else 0
    return tuple(ndata)


def run_funcs(b, c, a):
    funcs = [addr, addi, mulr, muli,
             banr, bani, borr, bori,
             setr, seti, gtir, gtri,
             gtrr, eqir, eqri, eqrr]

    count = 0
    for f in funcs:
        if f(c, b) == a:
            count += 1

    return count


def part1():
    count = 0
    for b, c, a in read_op_code_data():
        if run_funcs(b, c, a) >= 3:
            count += 1
    return count


def form_op_code_sets(b, c, a):
    names = ['addr', 'addi', 'mulr', 'muli',
             'banr', 'bani', 'borr', 'bori',
             'setr', 'seti', 'gtir', 'gtri',
             'gtrr', 'eqir', 'eqri', 'eqrr']
    funcs = [addr, addi, mulr, muli,
             banr, bani, borr, bori,
             setr, seti, gtir, gtri,
             gtrr, eqir, eqri, eqrr]

    b, c, a = tuple(b), tuple(c), tuple(a)
    right_funcs = set()
    for n, f in zip(names, funcs):
        if f(c, b) == a:
            right_funcs.add(n)

    return right_funcs


def work_out_op_code():
    possibles: Dict[str, set] = {i: None for i in range(16)}

    for b, c, a in read_op_code_data():
        op = c[0]
        pos = form_op_code_sets(b, c, a)
        if possibles[op] is None:
            possibles[op] = pos
        else:
            possibles[op] &= pos

    trial_dict = {}
    while len(trial_dict) < 16:
        key, value = -1, -1
        for k, v in possibles.items():
            if len(v) == 1:
                key, value = k, v.pop()
                break

        trial_dict[key] = value
        possibles.pop(key)
        for k in possibles.keys():
            possibles[k] -= {value}

    names = ['addr', 'addi', 'mulr', 'muli',
             'banr', 'bani', 'borr', 'bori',
             'setr', 'seti', 'gtir', 'gtri',
             'gtrr', 'eqir', 'eqri', 'eqrr']
    funcs = [addr, addi, mulr, muli,
             banr, bani, borr, bori,
             setr, seti, gtir, gtri,
             gtrr, eqir, eqri, eqrr]

    return {k: funcs[names.index(v)] for k, v in trial_dict.items()}


def part2():
    func_dict = work_out_op_code()
    test_codes = read_test_code()
    res = (0, 0, 0, 0)
    for cmd in test_codes:
        op = cmd[0]
        res = func_dict[op](cmd, res)
    return res

