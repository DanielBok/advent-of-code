import re
import numpy as np


def read_data(file='d23'):
    if not file.endswith('.txt'):
        file += '.txt'

    reg = re.compile(r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)')
    with open(file) as f:
        return np.array([tuple(int(i) for i in line) for line in reg.findall(f.read().strip())])


def part1():
    data = read_data('d23')
    x, y, z, max_signal = data[np.where(data[:, 3] == data[:, 3].max())][0]
    return np.sum(np.abs(data[:, :3] - [x, y, z]).sum(1) <= max_signal)



def part2():
    # < 120021045
    data = read_data('d23b')
    coords, radii = data[:, :3], data[:, 3]

    dist = np.array([np.sum(np.abs(coords - c).sum(1) <= radii) for c in coords])
    start = coords[dist == dist.max()]
    start = start[np.abs(start).sum(1) == np.abs(start).sum(1).max()][0]

    def calc_num_points(pt):
        return np.sum(np.abs(coords - pt).sum(1) <= radii)

    def next_coords(pt, s=1):
        x, y, z = pt
        return (x - s, y - s, z - s), (x - s, y, z), (x, y - s, z), (x, y, z - s)

    in_radii = calc_num_points(start)
    curr_in_radii = in_radii
    curr_pt = start
    while curr_in_radii >= in_radii:
        for c in next_coords(curr_pt):
            curr_in_radii = calc_num_points(c)
            if curr_in_radii >= in_radii:
                in_radii = curr_in_radii
                curr_pt = c
                break
    return curr_pt
