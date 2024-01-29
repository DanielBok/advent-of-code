from collections import defaultdict
import re

reg = re.compile('\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] ([\w\W]+)')

with open('d04.txt') as f:
    data = sorted(f.read().strip().split('\n'))

repo = defaultdict(int)
mm = 0
guard = 0
for line in data:
    comps = reg.findall(line)[0]
    year, month, day, hour, minute = [int(i) for i in comps[:-1]]
    cmd: str = comps[-1]
    if cmd.startswith('Guard'):
        guard = int(re.findall('#(\d+)', cmd)[0])
    elif cmd == 'falls asleep':
        mm = minute
        if hour == 23:
            mm -= 60
    elif cmd == 'wakes up':
        slept = minute - mm
        repo[guard] += slept
    else:
        raise ValueError(f'unknown cmd: {cmd}')

max_guard = 0
max_sleep = 0
for k, v in repo.items():
    if v > max_sleep:
        max_sleep = v
        max_guard = k

is_guard = False
time_repo = [0] * 60

for line in data:
    comps = reg.findall(line)[0]
    year, month, day, hour, minute = [int(i) for i in comps[:-1]]
    cmd: str = comps[-1]

    if cmd.startswith('Guard'):
        guard = int(re.findall('#(\d+)', cmd)[0])
        is_guard = guard == max_guard

    if not is_guard:
        continue

    if cmd == 'falls asleep':
        mm = minute
        if hour == 23:
            mm = 0
    elif cmd == 'wakes up':
        for i in range(mm, minute):
            time_repo[i] += 1

for i, j in enumerate(time_repo):
    if j == max(time_repo):
        print(i * max_guard)
        break


repo2 = {i: [0] * 60 for i in repo.keys()}
mm = 0
guard = 0

for line in data:
    comps = reg.findall(line)[0]
    year, month, day, hour, minute = [int(i) for i in comps[:-1]]
    cmd: str = comps[-1]

    if cmd.startswith('Guard'):
        guard = int(re.findall('#(\d+)', cmd)[0])
    elif cmd == 'falls asleep':
        mm = minute
    elif cmd == 'wakes up':
        for i in range(mm, minute):
            repo2[guard][i] += 1

max_guard2 = 0
max_time2 = 0
max_tt = 0

for guard, times in repo2.items():
    mm = max(times)
    if mm <= max_time2:
        continue
    else:
        max_guard2 = guard
        max_time2 = mm

    for i, t in enumerate(times):
        if t == mm:
            max_tt = i

print(max_guard2 * max_tt)
