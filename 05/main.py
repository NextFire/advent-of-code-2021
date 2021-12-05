import os
import numpy as np

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

splitted = INPUT.splitlines()
vents = [
    [tuple(map(int, ss.split(','))) for ss in s.split(' -> ')] for s in splitted
]


def part_one():
    diagram = np.zeros((1000, 1000), dtype=int)
    for vent in vents:
        compared_x = (vent[0][0], vent[1][0])
        compared_y = (vent[0][1], vent[1][1])
        if vent[0][0] == vent[1][0]:
            for j in range(min(*compared_y), max(*compared_y) + 1):
                diagram[vent[0][0], j] += 1
        if vent[0][1] == vent[1][1]:
            for i in range(min(*compared_x), max(*compared_x) + 1):
                diagram[i, vent[0][1]] += 1
    return len(np.where(diagram >= 2)[0])


def part_two():
    diagram = np.zeros((1000, 1000), dtype=int)
    for vent in vents:
        compared_x = (vent[0][0], vent[1][0])
        compared_y = (vent[0][1], vent[1][1])
        if vent[0][0] == vent[1][0]:
            for j in range(min(*compared_y), max(*compared_y) + 1):
                diagram[vent[0][0], j] += 1
        elif vent[0][1] == vent[1][1]:
            for i in range(min(*compared_x), max(*compared_x) + 1):
                diagram[i, vent[0][1]] += 1
        elif abs(vent[0][0] - vent[1][0]) == abs(vent[0][1] - vent[1][1]):
            diff = abs(vent[0][0] - vent[1][0])
            vent.sort(key=lambda x: x[0])
            if vent[1][1] > vent[0][1]:
                for k in range(diff + 1):
                    diagram[vent[0][0] + k, vent[0][1] + k] += 1
            else:
                for k in range(diff + 1):
                    diagram[vent[0][0] + k, vent[0][1] - k] += 1

    return len(np.where(diagram >= 2)[0])


if __name__ == '__main__':
    print(part_one())
    print(part_two())