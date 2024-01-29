import re


def read_data():
    with open('d21.txt') as f:
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


def bani(data, items):
    a, b, c = items
    data[c] = data[a] & b


def bori(data, items):
    a, b, c = items
    data[c] = data[a] | b


def eqri(data, items):
    a, b, c = items
    data[c] = int(data[a] == b)


def eqrr(data, items):
    a, b, c = items
    data[c] = int(data[a] == data[b])


def gtir(data, items):
    a, b, c = items
    data[c] = int(a > data[b])


def gtrr(data, items):
    a, b, c = items
    data[c] = int(data[a] > data[b])


def muli(data, items):
    a, b, c = items
    data[c] = data[a] * b


def seti(data, items):
    a, _, c = items
    data[c] = a


def setr(data, items):
    a, _, c = items
    data[c] = data[a]


fn_map = {
    'addi': addi,
    'addr': addr,
    'bani': bani,
    'bori': bori,
    'eqri': eqri,
    'eqrr': eqrr,
    'gtir': gtir,
    'gtrr': gtrr,
    'muli': muli,
    'seti': seti,
    'setr': setr,
}


def part1():
    register, instructions, p = read_data()
    special_line_index = 29  # must check from the code

    while True:
        index = register[p]
        if index < 0 or index >= len(instructions):
            break

        if index == special_line_index:
            return register[3]

        inst, *items = instructions[index]
        fn_map[inst](register, items)
        register[p] += 1


def part2():
    r3 = 0  # default
    r3_seen = set()
    r4_seen = set()
    last_seen = 0

    while True:
        r4 = r3 | 65536
        r3 = 10649702

        if r4 in r4_seen:
            break
        else:
            r4_seen.add(r4)

        while True:
            # ins 7 - 12
            r5 = r4 & 255
            r3 += r5
            r3 &= 16777215
            r3 *= 65899
            r3 &= 16777215

            # ins 13, break out of loop check else increment r4
            if 256 > r4:
                if r3 not in r3_seen:
                    last_seen = r3
                r3_seen.add(r3)

                break

            # ins 17 - 20, loops get r1 bigger by 256* until it is bigger than r4
            # then set r4 = r5 [r1 // 256]. so essentially divide by 256
            r4 //= 256

    return last_seen
