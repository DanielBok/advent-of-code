from collections import deque
from typing import Dict, List, Tuple

Pos = Tuple[int, int]
Grid = List[List[str]]


def read_data(file: str):
    if not file.endswith('.txt'):
        file += '.txt'
    with open(file) as f:
        return [list(line) for line in f.read().split('\n')]


board_changed = True
grid: Grid = read_data('d15')
actors: Dict[Pos, 'Actor'] = {}


class Actor:
    def __init__(self, _type: str):
        self.type = _type
        self.health = 200
        self.enemy = 'E' if self.type == 'G' else 'G'

    def attack_enemy(self, p: Pos):
        global grid, actors

        enemy = None
        enemy_pos = None
        for i, j in get_sides(p):
            if grid[i][j] == self.enemy:

                new_enemy = actors[(i, j)]
                if enemy is None or (0 <= new_enemy.health < enemy.health):
                    enemy = new_enemy
                    enemy_pos = (i, j)

        if enemy is not None:
            enemy.health -= 3
            if enemy.health <= 0:
                i, j = enemy_pos
                grid[i][j] = '.'
                # actors.pop(enemy_pos)

        return enemy is not None

    def find_closest_enemy(self, p: Pos) -> Pos:
        global grid, actors

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


def game_ended(enemy_type: str = None):
    global actors

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


def score(rounds):
    global actors
    return sum(a.health for a in actors.values()) * rounds


def print_grid(row_num=None):
    global grid

    if row_num is not None:
        print(''.join(grid[row_num]))
    else:
        print('\n'.join(''.join(row) for row in grid))


for x, row in enumerate(grid):
    for y, e in enumerate(row):
        if e == 'G' or e == 'E':
            actors[(x, y)] = Actor(e)

c = 0

while True:
    order = sorted(actors.keys())
    has_moved = []
    break_out = False
    for pos in order:
        actor = actors.get(pos, None)
        if game_ended(actor.enemy):
            break_out = True
            break

        if actor is None or actor.health <= 0:
            continue

        if actor.attack_enemy(pos):
            continue

        if not board_changed:
            continue

        np = actor.find_closest_enemy(pos)  # new position

        if np == pos:
            continue

        grid[pos[0]][pos[1]] = '.'
        grid[np[0]][np[1]] = actor.type
        actor.attack_enemy(np)
        actors[np] = actors.pop(pos)
        has_moved.append(True)

    if break_out or game_ended():
        break

    keys = list(actors.keys())
    for pos in keys:
        if actors[pos].health <= 0:
            actors.pop(pos)
            has_moved.append(True)

    board_changed = any(has_moved)
    c += 1

print(score(c))
# has enemy
# if no:
#   move
#   has enemy
#   if yes: attack
# else: attack
