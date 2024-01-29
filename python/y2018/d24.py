import re

IMMUNITY = 'immunity'
VIRUS = 'virus'


class Unit:
    def __init__(self, id_, class_, n, hp, type_, dmg, init, immune, weak):
        self.id = id_
        self.class_ = class_
        self.n = n
        self.hp = hp
        self.dmg = dmg
        self.type = type_
        self.init = init
        self.immune = immune
        self.weak = weak
        self.target = None

    @property
    def power(self):
        return self.n * self.dmg

    def damage_to(self, enemy):
        if self.type in enemy.immune or self.class_ == enemy.class_:
            return 0
        elif self.type in enemy.weak:
            return 2 * self.power
        else:
            return self.power

    def __str__(self):
        return f'<Unit {self.class_} - (id: {self.id}, n: {self.n}, hp: {self.hp}, power: {self.power}, init: {self.init}), dmg: {self.dmg}>'

    def __repr__(self):
        return f'<Unit {self.class_} - (id: {self.id}, n: {self.n}, hp: {self.hp}, power: {self.power}, init: {self.init}), dmg: {self.dmg}>'


def read_data(file='d24a'):
    if not file.endswith('.txt'):
        file += '.txt'

    def parse_info(id_: int, info: str, is_virus: bool):
        info = info.strip()
        weakness = re.findall('\([\w ;,]+\)', info)
        if len(weakness) > 0:
            weakness = weakness.pop()
            info = info.replace(weakness, '')
            weakness = weakness.replace('(', '').replace(')', '')

        values = {'id_': id_, 'class_': VIRUS if is_virus else IMMUNITY, 'immune': [], 'weak': []}
        for i, e in zip(['n', 'hp', 'dmg', 'type_', 'init'], info.split()):
            if i != 'type_':
                e = int(e)
            values[i] = e

        if weakness:
            for w in weakness.split(';'):
                w = w.strip()
                i = 'weak' if w.startswith('weak') else 'immune'
                values[i] = [e.strip() for e in w.replace(f'{i} to ', '').split(',')]

        return Unit(**values)

    with open(file) as f:
        units = []
        is_virus = False
        i = 1
        for line in f.readlines():
            if line.strip() == '' or line.startswith('Immune'): continue
            if line.startswith('Infection'):
                is_virus = True
                i = 1
                continue

            units.append(parse_info(i, line, is_virus))
            i += 1

    return units


class Simulation:
    def __init__(self, file='d24a', boost=0):
        self.units = read_data(file)

        for u in self.units:
            if u.class_ == IMMUNITY:
                u.dmg += boost

    def attack(self):
        self.units.sort(key=lambda d: (-d.power, -d.init))
        defenders = {IMMUNITY: set(), VIRUS: set()}

        for u in self.units:
            assert u.n > 0

            def_set = defenders[u.class_]
            targets = sorted([e for e in self.units if
                              e.class_ != u.class_ and u.damage_to(e) > 0 and e.id not in def_set],
                             key=lambda e: (-u.damage_to(e), -e.power, -e.init))

            if targets:
                u.target = targets[0]
                def_set.add(u.target.id)
            else:
                u.target = None

        self.units.sort(key=lambda u: -u.init)
        units_killed = 0
        for u in self.units:
            if u.target is None:
                continue
            loss = u.damage_to(u.target) // u.target.hp
            u.target.n = max(0, u.target.n - loss)
            units_killed += loss

        if units_killed == 0:
            return False

        self.units = [u for u in self.units if u.n > 0]
        return True

    def _run(self):
        while len({u.class_ for u in self.units}) > 1:
            okay = self.attack()
            if not okay:
                print('# For the particular boost, we are stuck in a loop where 0 units are killed')
                return False

        return True

    @property
    def units_left(self):
        if not self._run():
            return None

        return sum(u.n for u in self.units)

    @property
    def winner(self):
        if not self._run():
            return None

        for u in self.units:
            return u.class_


def test_part1():
    sim = Simulation()
    assert sim.units_left == 5216


def part1():
    sim = Simulation('d24')
    print(sim.units_left)


def part2():
    i = 1
    while True:
        sim = Simulation('d24', boost=i)
        if sim.winner == IMMUNITY:
            print('First solution found at boost = ', i)
            print('Units left = ', sim.units_left)
            return sim.units_left
        i += 1
