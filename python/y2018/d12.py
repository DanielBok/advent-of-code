with open('d12.txt') as f:
    raw = f.read().strip().split('\n')
    init = raw[0].split(': ')[-1].strip()
    dmap = {i: j for i, j in [n.split(' => ') for n in raw[2:]]}

t = 500
ext = (t + 1)
line = '.' * ext + init + '.' * ext

values = [line]


def get_num(l):
    return sum(i - ext for i, j in enumerate(l) if j == '#')


nums = [get_num(line)]

for i in range(t):
    line = '..' + ''.join(dmap[line[j - 2:j + 3]] for j in range(2, len(line) - 2)) + '..'
    values.append(line)
    nums.append(get_num(line))

# s = sum(i - ext for i, j in enumerate(line) if j == '#')
# print(s)

with open('d12temp.txt', 'w') as f:
    for v in values:
        f.write(str(v).replace('.', '_')[500:] + '\n')


s = '###_#____###___###____###______###____###____###____###____###_______###____###______###___###________###______###___________###______###____###___###___###_______###___###___###___###'


magic_no = 162 - 76
sum(i + 50000000000 - magic_no for i, j in enumerate(s) if j == '#')