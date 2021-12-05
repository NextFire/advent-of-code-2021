import io
import os
import numpy as np

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

splitted = INPUT.split('\n\n')
drawed = [int(v) for v in splitted[0].split(',')]
boards = [np.loadtxt(io.StringIO(s)) for s in splitted[1:]]


def part_one():
    checkboards = [np.full_like(boards[0], False) for _ in boards]
    for d in drawed:
        for n, b in enumerate(boards):
            for i, j in zip(*np.where(b == d)):
                checkboards[n][i, j] = True
            if any(
                    all(l) for l in [
                        checkboards[n][i, :]
                        for i in range(np.shape(checkboards[n])[0])
                    ] + [
                        checkboards[n][:, j]
                        for j in range(np.shape(checkboards[n])[1])
                    ]):
                filtered = boards[n] - checkboards[n] * boards[n]
                return np.sum(filtered) * d


def part_two():
    checkboards = [np.full_like(boards[0], False) for _ in boards]
    winning = []
    for d in drawed:
        for n, b in enumerate(boards):
            for i, j in zip(*np.where(b == d)):
                checkboards[n][i, j] = True
            if any(
                    all(l) for l in [
                        checkboards[n][i, :]
                        for i in range(np.shape(checkboards[n])[0])
                    ] + [
                        checkboards[n][:, j]
                        for j in range(np.shape(checkboards[n])[1])
                    ]) and n not in winning:
                winning.append(n)
                if len(winning) == len(boards):
                    filtered = boards[n] - checkboards[n] * boards[n]
                    return np.sum(filtered) * d


if __name__ == "__main__":
    print(part_one())
    print(part_two())
