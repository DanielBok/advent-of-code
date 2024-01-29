import re

reg = re.compile('#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')
with open('d03.txt') as f:
    commands = [[int(j) for j in reg.findall(i)[0]] for i in f]

max_x, max_y, long_x, long_y = [max(i[j] for i in commands) + 1 for j in range(1, 5)]

max_x += long_x
max_y += long_y

grid = [[0 for _ in range(max_x)] for _ in range(max_y)]

for _, x, y, xp, yp in commands:
    for i in range(x, x + xp):
        for j in range(y, y + yp):
            grid[j][i] += 1

count = 0
for row in grid:
    for e in row:
        if e > 1:
            count += 1

for c, x, y, xp, yp in commands:
    passed = True
    for i in range(x, x + xp):
        for j in range(y, y + yp):
            if grid[j][i] != 1:
                passed = False
                break

        if not passed:
            break

    if passed:
        print(c)
        break
