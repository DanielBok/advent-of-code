from string import ascii_lowercase

with open('d05.txt') as f:
    string = f.read().strip()


def cancels(l1: str, l2: str) -> bool:
    return l1 != l2 and (l1.lower() == l2 or l1.upper() == l2)


def q1(_string: str):
    parcel = []
    for l in _string:
        if len(parcel) == 0 or not cancels(parcel[-1], l):
            parcel.append(l)
        else:
            parcel.pop()

    return ''.join(parcel)


print(len(q1(string)))


def q2(_string):
    shortest = len(_string) + 1
    letter = ''
    for l in ascii_lowercase:
        _s = string.replace(l, '').replace(l.upper(), '')
        _len = len(q1(_s))
        if _len < shortest:
            letter = l
            shortest = _len

    print('Shortest letter is: ', letter)
    return shortest


q2(string)