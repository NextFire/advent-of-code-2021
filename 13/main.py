import os
from collections import defaultdict
from functools import reduce
from itertools import chain
import re

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

blocks = INPUT.split('\n\n')
dotlist = [tuple(int(a) for a in l.split(',')) for l in blocks[0].splitlines()]
reg = re.compile(r' (.)=(.*)')
folds = []
for l in blocks[1].splitlines():
    (a, b) = reg.search(l).groups()
    folds.append((a, int(b)))


def part_one():
    dtl = dotlist
    for dir, n in folds[:1]:
        dts = set()
        for d in dtl:
            if dir == 'x':
                if d[0] >= n and d[0] <= 2 * n:
                    dts.add((d[0] - 2 * (d[0] - n), d[1]))
                elif d[0] < n:
                    dts.add(d)
            else:
                if d[1] >= n and d[1] <= 2 * n:
                    dts.add((d[0], d[1] - 2 * (d[1] - n)))
                elif d[1] < n:
                    dts.add(d)
        dtl = dts
    return len(dtl)


def part_two():
    dtl = dotlist
    for dir, n in folds:
        dts = set()
        for d in dtl:
            if dir == 'x':
                if d[0] >= n and d[0] <= 2 * n:
                    dts.add((d[0] - 2 * (d[0] - n), d[1]))
                elif d[0] < n:
                    dts.add(d)
            else:
                if d[1] >= n and d[1] <= 2 * n:
                    dts.add((d[0], d[1] - 2 * (d[1] - n)))
                elif d[1] < n:
                    dts.add(d)
        dtl = dts
    paper = [['.' for _ in range(50)] for _ in range(10)]
    for (i, j) in dtl:
        paper[j][i] = '#'
    return '\n'.join([''.join(l) for l in paper])


if __name__ == '__main__':
    print(part_one())
    print(part_two())
