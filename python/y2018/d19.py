import re


def read_data():
    with open('d19.txt') as f:
        raw = f.read().strip()
    instructions = [(line[0], *(int(i) for i in line[1:])) for line in
                    re.findall(r'(\w+) (\d+) (\d+) (\d+)', raw.strip())]

    pointer = int(raw.split('\n')[0].split()[-1])
    register = [0] * 6

    return register, instructions, pointer


def addi(data, items):
    a, b, c = items
    data[c] = data[a] + b


def addr(data, items):
    a, b, c = items
    data[c] = data[a] + data[b]


def muli(data, items):
    a, b, c = items
    data[c] = data[a] * b


def mulr(data, items):
    a, b, c = items
    data[c] = data[a] * data[b]


def seti(data, items):
    a, _, c = items
    data[c] = a


def setr(data, items):
    a, _, c = items
    data[c] = data[a]


def gtrr(data, items):
    a, b, c = items
    data[c] = int(data[a] > data[b])


def eqrr(data, items):
    a, b, c = items
    data[c] = int(data[a] == data[b])


fn_map = {
    'addi': addi,
    'addr': addr,
    'muli': muli,
    'mulr': mulr,
    'seti': seti,
    'setr': setr,
    'eqrr': eqrr,
    'gtrr': gtrr
}


# part 1
def part1():
    register, instructions, p = read_data()
    while True:
        index = register[p]
        if index < 0 or index >= len(instructions):
            break

        inst, *items = instructions[index]
        fn_map[inst](register, items)
        register[p] += 1

    return register


def part1_fast():
    f = 4 * 19 * 11 + 22 * 4 + 21  # f is a loop guard
    return sum(i + 1 for i in range(f) if f % (i + 1) == 0)


def part2():
    # f is formed from inst 17 to 36
    f = 4 * 19 * 11 + 22 * 4 + 21  # f is a loop guard, first loop, lines 17 - 24
    f += (27 * 28 + 29) * 30 * 14 * 32  # lines 27 - 33

    return sum(i + 1 for i in range(f) if f % (i + 1) == 0)
