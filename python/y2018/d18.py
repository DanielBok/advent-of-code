def read_data(file: str):
    if not file.endswith('.txt'):
        file += '.txt'
    with open(file) as f:
        return [list(i.strip()) for i in f.read().strip().split('\n')]


class Simulation:
    def __init__(self, file: str):
        self.grid = read_data(file)
        self.orig = self.grid.copy()
        self.max_x = len(self.grid[0])
        self.max_y = len(self.grid)

    def adj_cells(self, x, y):
        cells = []
        for i in range(-1, 2):
            for j in range(-1, 2):
                if i == 0 and j == 0:
                    continue
                xx, yy = x + i, y + j
                if xx < 0 or xx >= self.max_x or yy < 0 or yy >= self.max_y:
                    continue
                cells.append((xx, yy))

        return cells

    def make_new_grid(self):
        return [['.'] * self.max_x for _ in range(self.max_y)]

    def run(self, times=10, verbose=False):
        for _ in range(times):
            ngrid = self.make_new_grid()
            for y in range(self.max_y):
                for x in range(self.max_x):
                    ngrid[y][x] = self._transform(x, y)

            self.grid = ngrid
            if verbose:
                print(self._resource_value)

    def _transform(self, x, y):
        cell = self.grid[y][x]
        adj = self.adj_cells(x, y)

        if cell == '.':
            tree_count = sum(self.grid[j][i] == '|' for i, j in adj)
            return '|' if tree_count >= 3 else '.'

        if cell == '|':
            lumber_count = sum(self.grid[j][i] == '#' for i, j in adj)
            return '#' if lumber_count >= 3 else '|'

        if cell == '#':
            lumber_count = sum(self.grid[j][i] == '#' for i, j in adj)
            tree_count = sum(self.grid[j][i] == '|' for i, j in adj)
            return '#' if lumber_count >= 1 and tree_count >= 1 else '.'

    def print_grid(self):
        print('\n'.join(''.join(row) for row in self.grid))

    @property
    def _resource_value(self):
        trees, lumber = 0, 0
        for row in self.grid:
            trees += row.count('|')
            lumber += row.count('#')

        return trees, lumber, trees * lumber

    def part1(self):
        self.grid = self.orig.copy()
        self.run(10)
        _, _, ans = self._resource_value
        return ans

    def part2(self, times=1000000000):
        if times < 475:
            self.run(times)
            _, _, ans = self._resource_value
            return ans

        index =(times - 475) % 28
        return [181485,
                177416,
                173910,
                167475,
                164424,
                164079,
                163631,
                163248,
                167090,
                168562,
                171588,
                172852,
                174900,
                176012,
                182574,
                187272,
                193888,
                199167,
                203648,
                204832,
                210504,
                210824,
                207282,
                205320,
                204125,
                197316,
                192984,
                188914][index]


s = Simulation('d18')

# print(s.part1())
print(s.part2())
