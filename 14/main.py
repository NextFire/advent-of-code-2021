import os
from collections import Counter

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

blocks = INPUT.split('\n\n')
template = blocks[0]
pairs = {}
for l in blocks[1].splitlines():
    a, b = l.split(' -> ')
    pairs[a] = b


def part_one():
    polymer = list(template)
    for _ in range(10):
        newp = []
        for i in range(1, len(polymer)):
            d, f = polymer[i - 1], polymer[i]
            newp.append(d)
            newp.append(pairs.get(d + f))
        newp.append(f)
        polymer = list(filter(None, newp))
    count = Counter(polymer)
    return count.most_common()[0][1] - count.most_common()[-1][1]


def part_two():
    ...


if __name__ == '__main__':
    print(part_one())
    print(part_two())
