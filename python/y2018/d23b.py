# Copyright (c) Microsoft Corporation 2015, 2016

# The Z3 Python API requires libz3.dll/.so/.dylib in the 
# PATH/LD_LIBRARY_PATH/DYLD_LIBRARY_PATH
# environment variable and the PYTHON_PATH environment variable
# needs to point to the `python' directory that contains `z3/z3.py'
# (which is at bin/python in our binary releases).

# If you obtained d23b.py as part of our binary release zip files,
# which you unzipped into a directory called `MYZ3', then follow these
# instructions to run the example:

# Running this example on Windows:
# set PATH=%PATH%;MYZ3\bin
# set PYTHONPATH=MYZ3\bin\python
# python d23b.py

# Running this example on Linux:
# export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:MYZ3/bin
# export PYTHONPATH=MYZ3/bin/python
# python d23b.py

# Running this example on macOS:
# export DYLD_LIBRARY_PATH=$DYLD_LIBRARY_PATH:MYZ3/bin
# export PYTHONPATH=MYZ3/bin/python
# python d23b.py


import re

from z3 import *


def read_data():
    file = r'C:\Projects\AOC\d23.txt'

    reg = re.compile(r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)')
    with open(file) as f:
        return [tuple(int(i) for i in line) for line in reg.findall(f.read().strip())]


nanobots = read_data()
lenr = list(range(len(nanobots)))


def zabs(x):
    return If(x >= 0, x, -x)


(x, y, z) = (Int('x'), Int('y'), Int('z'))
in_ranges = [Int('in_range_' + str(i)) for i in lenr]
range_count = Int('sum')
o = Optimize()
for i in lenr:
    nx, ny, nz, nrng = nanobots[i]
    o.add(in_ranges[i] == If(zabs(x - nx) + zabs(y - ny) + zabs(z - nz) <= nrng, 1, 0))

o.add(range_count == sum(in_ranges))
dist_from_zero = Int('dist')
o.add(dist_from_zero == zabs(x) + zabs(y) + zabs(z))
h1 = o.maximize(range_count)
h2 = o.minimize(dist_from_zero)

print(o.check())
print(o.lower(h2), o.upper(h2))
