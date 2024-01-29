from collections import defaultdict

with open('d02.txt') as f:
    letters = [i.strip() for i in f.read().strip().split()]


counts = defaultdict(int)

for line in letters:
    d = defaultdict(int)
    for l in line:
        d[l] += 1

    imap = {}

    for v in d.values():
        imap[v] = 1

    for k, v in imap.items():
        if k == 1:
            continue
        counts[k] += 1

value = 1
for v in counts.values():
    value *= v

print(value)

ll = len(letters)
ll2 = len(letters[0])

for i in range(ll):
    o = letters[i]
    for j in range(i + 1, ll):
        diff = 0
        n = letters[j]
        for k in range(ll2):
            if n[k] != o[k]:
                diff += 1

            if diff > 1:
                break
        if diff == 1:
            ans = ''
            for k in range(ll2):
                if n[k] == o[k]:
                    ans += n[k]
            print(ans)
            raise ValueError
else:
    print('no answer')

