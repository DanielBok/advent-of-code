import os

os.chdir('AOC')

P = 939601


def q1(num: int):
    p1 = 0
    p2 = 1

    data = [3, 7]

    while len(data) < num + 10:
        v1, v2 = data[p1], data[p2]
        d = v1 + v2

        if d < 10:
            data.append(d)
        else:
            data.extend([int(d / 10), d % 10])

        p1 = (p1 + v1 + 1) % len(data)
        p2 = (p2 + v2 + 1) % len(data)

    return ''.join(str(i) for i in data[num:num + 10])


def test_q1():
    assert q1(5) == '0124515891'
    assert q1(9) == '5158916779'
    assert q1(18) == '9251071085'
    assert q1(2018) == '5941429882'


test_q1()
print(q1(P))


def q2(num):
    p1 = 0
    p2 = 1

    data = [3, 7]
    num = str(num)

    c, n = 0, len(num)

    while True:
        v1, v2 = data[p1], data[p2]
        d = v1 + v2

        if d < 10:
            data.append(d)
        else:
            data.extend([int(d / 10), d % 10])

        p1 = (p1 + v1 + 1) % len(data)
        p2 = (p2 + v2 + 1) % len(data)

        if len(data) < n:
            continue

        while c + n < len(data):
            value = ''.join(str(i) for i in data[c:c + n])
            if value == num:
                return c
            c += 1


def test_q2():
    assert q2(51589) == 9
    assert q2('01245') == 5


assert q2(92510) == 18
assert q2(59414) == 2018

test_q2()
print(q2(P))
