from collections import deque
from typing import Dict, List, Tuple

Pos = Tuple[int, int]
Grid = List[List[str]]


class Simulation:
    def __init__(self, file: str, elf_power=3):

        self.elf_power = elf_power
        self.board_changed = True

        if not file.endswith('.txt'):
            file += '.txt'

        self._file = file
        self.actors: Dict[Pos, 'Actor'] = {}
        self.rounds = 0
        self.grid: Grid = []
        self.reset()

    def reset(self):
        with open(self._file) as f:
            self.grid = [list(line) for line in f.read().split('\n')]

        self.actors: Dict[Pos, 'Actor'] = {}
        for x, row in enumerate(self.grid):
            for y, e in enumerate(row):
                if e == 'G' or e == 'E':
                    self.actors[(x, y)] = Actor(e)

        self.rounds = 0
        self.board_changed = True

    def game_ended(self, enemy_type: str = None):
        actors = self.actors

        if enemy_type is not None:
            for a in actors.values():
                if enemy_type == a.type:
                    return False
            return True
        else:
            s = set()
            for a in actors.values():
                s.add(a.type)

            return len(s) == 1

    def print_grid(self, row_num=None):
        grid = self.grid

        if row_num is not None:
            print(''.join(grid[row_num]))
        else:
            print('\n'.join(''.join(row) for row in grid))

    def part1(self):
        self.run_game()
        actors = self.actors
        return sum(a.health for a in actors.values()) * self.rounds

    def part2(self, elf_power=15):
        self.elf_power = elf_power
        self.reset()
        starting_elves = sum(a.type == 'E' for a in self.actors.values())
        self.run_game()

        ending_elves = sum(a.type == 'E' for a in self.actors.values())
        if starting_elves == ending_elves:
            print('No elves died')
            return sum(a.health for a in self.actors.values()) * self.rounds
        print('Some elves died')
        return 0

    def run_game(self):
        while True:
            order = sorted(self.actors.keys())
            has_moved = []
            for pos in order:
                actor = self.actors.get(pos, None)
                if self.game_ended(actor.enemy):
                    return

                if actor is None or actor.health <= 0:
                    continue

                if actor.attack_enemy(pos, self.grid, self.actors, self.elf_power):
                    continue

                if not self.board_changed:
                    continue

                np = actor.find_closest_enemy(pos, self.grid)  # new position

                if np == pos:
                    continue

                self.grid[pos[0]][pos[1]] = '.'
                self.grid[np[0]][np[1]] = actor.type
                actor.attack_enemy(np, self.grid, self.actors, self.elf_power)
                self.actors[np] = self.actors.pop(pos)
                has_moved.append(True)

            keys = list(self.actors.keys())
            for pos in keys:
                if self.actors[pos].health <= 0:
                    self.actors.pop(pos)
                    has_moved.append(True)

            if self.game_ended():
                return

            self.board_changed = any(has_moved)
            self.rounds += 1


class Actor:
    def __init__(self, _type: str):
        self.type = _type
        self.health = 200
        self.enemy = 'E' if self.type == 'G' else 'G'

    def attack_enemy(self, p: Pos, grid: Grid, actors: Dict[Pos, 'Actor'], power_level=3):

        enemy = None
        enemy_pos = None
        for i, j in get_sides(p):
            if grid[i][j] == self.enemy:

                new_enemy = actors[(i, j)]
                if enemy is None or (0 <= new_enemy.health < enemy.health):
                    enemy = new_enemy
                    enemy_pos = (i, j)

        if enemy is not None:
            damage = power_level if self.type == 'E' else 3
            enemy.health -= damage
            if enemy.health <= 0:
                i, j = enemy_pos
                grid[i][j] = '.'
                # actors.pop(enemy_pos)

        return enemy is not None

    def find_closest_enemy(self, p: Pos, grid: Grid) -> Pos:

        start = p
        paths = {start: None}
        to_visit = deque([start])
        while True:
            if len(to_visit) == 0:
                return start

            p = to_visit.popleft()
            for (i, j) in get_sides(p):
                pp = (i, j)
                cell = grid[i][j]
                if cell == '#' or cell == self.type:
                    continue
                if cell == '.' and pp not in paths:
                    paths[pp] = p
                    to_visit.append(pp)
                if cell == self.enemy:
                    while True:
                        parent = paths[p]
                        if parent == start:
                            return p
                        p = parent

    def __repr__(self):
        _t = 'Elf' if self.type == 'E' else 'Goblin'
        return f'Actor: {_t}. Health {self.health}'


def get_sides(p: Pos):
    top = p[0] - 1, p[1]
    left = p[0], p[1] - 1
    right = p[0], p[1] + 1
    bottom = p[0] + 1, p[1]
    return top, left, right, bottom


sim = Simulation('d15')
# print(sim.part1())

for i in range(18, 21):
    print(sim.part2(i))
