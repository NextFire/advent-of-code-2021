import os

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

lines = list(map(int, INPUT.splitlines()))


def part_one():
    counter = 0
    for i, v in enumerate(lines):
        if i > 0 and v > lines[i - 1]:
            counter += 1
    return counter


def part_two():
    counter = 0
    for i in range(len(lines)):
        if i > 0 and i < len(lines) - 2:
            acc_a = lines[i - 1] + lines[i] + lines[i + 1]
            acc_b = lines[i] + lines[i + 1] + lines[i + 2]
            if acc_b > acc_a:
                counter += 1
    return counter


if __name__ == '__main__':
    print(part_one())
    print(part_two())
