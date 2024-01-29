import re

reg = re.compile(r'(-?\d+)')

with open('d10.txt') as f:
    data = [[int(j) for j in reg.findall(i)] for i in f.read().strip().split('\n')]

xs = [i[0] for i in data]
ys = [i[1] for i in data]
xv = [i[2] for i in data]
yv = [i[3] for i in data]

has_passed = False
ii = list(range(len(data)))
steps = 0

size = 70

while True:
    xs = [xs[i] + xv[i] for i in ii]
    ys = [ys[i] + yv[i] for i in ii]
    steps += 1

    x_min, x_max, y_min, y_max = min(xs), max(xs), min(ys), max(ys)
    if x_max - x_min >= size or y_max - y_min >= size:
        if has_passed:
            break
        continue

    has_passed = True
    grid = [['_' for _ in range(size)] for _ in range(size)]
    for i in ii:
        grid[ys[i] - y_min][xs[i] - x_min] = '#'

    print('-' * size)
    print(f'Step: {steps}', end='\n' * 3)
    for row in grid:
        if all(i == '_' for i in row):
            continue
        print(''.join(row))

    print('\n' * 3 + '-' * size, end='\n' * 3)
