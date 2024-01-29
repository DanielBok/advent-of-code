def read_path():
    with open('d20.txt') as f:
        s = f.read().strip()
    s = s.replace('|)', ')')
    return s


def next_position(curr, letter):
    x, y = curr
    if letter == 'N':
        return x, y + 1
    if letter == 'S':
        return x, y - 1
    if letter == 'E':
        return x + 1, y
    if letter == 'W':
        return x - 1, y


def build_dist_dict(path):
    curr = (0, 0)
    dist = {(0, 0): 0}
    stack = []

    for letter in path:
        if letter in {'N', 'S', 'E', 'W'}:
            next_pos = next_position(curr, letter)
            if next_pos not in dist:
                dist[next_pos] = dist[curr] + 1
            curr = next_pos
        elif letter == '(':
            stack.append(curr)
        elif letter == '|':
            curr = stack[-1]
        elif letter == ')':
            curr = stack.pop()

    return dist

def run_part1(path):
    dist = build_dist_dict(path)
    return max(dist.values())


def test_run_part1():
    for path, expected in [
        ("ENWWW(NEEE|SSE(EE|N))", 10),
        ("ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN", 18),
        ("ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))", 23),
        ("WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))", 31)
    ]:
        assert run_part1(path) == expected


def part1():
    return run_part1(read_path())


def part2():
    dist = build_dist_dict(read_path())
    return sum(i >= 1000 for i in dist.values())
