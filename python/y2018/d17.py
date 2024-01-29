import re


def read_data(file='d17'):
    if not file.endswith('.txt'):
        file += '.txt'

    with open(file) as f:
        raw = f.read()
        regx = re.compile(r'x=(\d+), y=(\d+)..(\d+)')
        regy = re.compile(r'y=(\d+), x=(\d+)..(\d+)')

        data_x = [tuple(int(i) for i in row) for row in regx.findall(raw)]
        data_y = [tuple(int(i) for i in row) for row in regy.findall(raw)]

    x_max = max(*[i[0] for i in data_x], *[i[2] for i in data_y])
    y_max = max(*[i[0] for i in data_y], *[i[2] for i in data_x])

    _grid = [['.'] * (x_max + 1) for _ in range(y_max + 1)]

    for x, ys, ye in data_x:
        for y in range(ys, ye + 1):
            _grid[y][x] = '#'

    for y, xs, xe in data_y:
        for x in range(xs, xe + 1):
            _grid[y][x] = '#'

    _grid[0][500] = '+'
    return _grid


class Simulation:
    def __init__(self, file='d17'):
        self.grid = read_data(file)
        self.drops = [(1, 500)]
        self.sides = []
        self.grid_len = len(self.grid)

    def run(self):
        start = True
        while start or len(self.drops) > 0:
            self._move_down()
            self._move_side()
            start = False

    def _move_down(self):
        if len(self.drops) == 0:
            return

        y, x = self.drops.pop()

        cell = self.grid[y][x]
        if (y + 1 == self.grid_len and cell == '|') or (cell == '|' and self.grid[y + 1][x] == '|'):
            return

        while y < self.grid_len:
            cell = self.grid[y][x]
            if cell == '.':
                self.grid[y][x] = '|'
                self.drops.append((y, x))
                self.sides.append((y, x))
                y += 1
            elif cell == '|':
                y += 1
            else:
                break

        # if y >= self.grid_len or self.grid[y][x] == '#':
        #     self.sides.append((y, x))

    def _move_side(self):
        if len(self.sides) == 0:
            return
        y, x = self.sides.pop()

        if y + 1 >= self.grid_len or self.grid[y + 1][x] in {'|', '.'}:
            return

        c = x + 1
        right_met_wall, right_stop = False, 0
        while c < len(self.grid[0]):
            if self.grid[y][c] == '#':
                right_met_wall = True
                right_stop = c
                break
            elif self.grid[y][c] == '.':
                self.grid[y][c] = '|'
                if self.grid[y + 1][c] == '.':  # floor is sand, drop
                    # means floor can only be sand
                    self.drops.append((y, c))
                    self.sides.append((y, c))
                    break
            elif self.grid[y][c] == '|':
                if self.grid[y + 1][c] == '.':
                    self.drops.append((y, c))
                    self.sides.append((y, c))
                    break
                elif self.grid[y + 1][c] == '|':
                    break
            c += 1

        c = x - 1
        left_met_wall, left_stop = False, 0
        while c >= 0:
            if self.grid[y][c] == '#':
                left_met_wall = True
                left_stop = c + 1
                break
            elif self.grid[y][c] == '.':
                self.grid[y][c] = '|'
                if self.grid[y + 1][c] == '.':  # floor is sand, drop
                    self.drops.append((y, c))
                    self.sides.append((y, c))
                    break
            elif self.grid[y][c] == '|':
                if self.grid[y + 1][c] == '.':
                    if self.grid[y + 1][c] == '.':
                        self.drops.append((y, c))
                        self.sides.append((y, c))
                        break
                elif self.grid[y + 1][c] == '|':
                    break
            c -= 1

        if right_met_wall and left_met_wall:
            for i in range(left_stop, right_stop):
                self.grid[y][i] = '~'

    def write_grid(self, file='d17_grid.txt'):
        with open(file, 'w') as f:
            f.write(self._str_grid)

    @property
    def _str_grid(self):
        skip = len(self.grid[0])
        for row in self.grid:
            num_dots = 0
            for i in row:
                if i == '.':
                    num_dots += 1
                else:
                    break
            skip = min(skip, num_dots)
        skip -= 1

        grid = ''
        for row in self.grid:
            grid += ''.join(row[skip:]) + '\n'

        return grid.strip()

    def part1(self):
        c = 0
        while '#' not in self.grid[c]:
            c += 1

        total = 0
        for i in range(c, self.grid_len):
            total += sum(self.grid[i].count(j) for j in ('|', '~'))

        return total

    def part2(self):
        total = 0
        for row in self.grid:
            total += row.count('~')
        return total


s = Simulation('d17')
s.run()
s.write_grid()

print(s.part1())
print(s.part2())

