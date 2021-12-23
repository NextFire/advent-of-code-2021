import os
from collections import defaultdict
from functools import reduce
from itertools import chain

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

lines = INPUT.splitlines()
links = [l.split('-') for l in lines]

graph: dict[str, list[str]] = defaultdict(list)
for l in links:
    graph[l[0]].append(l[1])
    graph[l[1]].append(l[0])


def part_one():

    def get_paths(path: list[str]):
        if path[-1] == 'end':
            return [path]

        next_nodes = [
            n for n in graph[path[-1]] if n.isupper() or n not in path
        ]

        return reduce(lambda x, y: x + y,
                      (get_paths(path + [n]) for n in next_nodes), [])

    parcours = get_paths(['start'])
    return len(parcours)


def part_two():

    def get_paths(path: list[str], optioned=False):
        if path[-1] == 'end':
            return [path]

        next_nodes = [
            n for n in graph[path[-1]] if n.isupper() or n not in path
        ]
        options = [
            n for n in graph[path[-1]]
            if n.islower() and n != 'start' and path.count(n) == 1
        ] if not optioned else []

        return reduce(
            lambda x, y: x + y,
            chain((get_paths(path + [n], optioned) for n in next_nodes),
                  (get_paths(path + [n], True) for n in options)), [])

    parcours = get_paths(['start'])
    return len(parcours)


if __name__ == '__main__':
    print(part_one())
    print(part_two())
