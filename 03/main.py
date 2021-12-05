import os

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

lines = INPUT.splitlines()


def part_one():
    gamma = 0
    epsilon = 0

    for i in range(len(lines[0])):
        nb_ones = sum([int(v[i]) for v in lines])
        is_one_most = nb_ones >= (len(lines) / 2)
        gamma = (gamma << 1) | int(is_one_most)
        epsilon = (epsilon << 1) | int(not is_one_most)

    return gamma * epsilon


def part_two():
    filtered = lines
    for i in range(len(lines[0])):
        nb_ones = sum([int(v[i]) for v in filtered])
        is_mostly_one = nb_ones >= (len(filtered) / 2)
        filtered = [v for v in filtered if int(v[i]) == int(is_mostly_one)]

        if len(filtered) == 1:
            oxygen = int(filtered[0], base=2)
            break

    filtered = lines
    for i in range(len(lines[0])):
        nb_ones = sum([int(v[i]) for v in filtered])
        is_mostly_one = nb_ones >= (len(filtered) / 2)
        filtered = [v for v in filtered if int(v[i]) == int(not is_mostly_one)]

        if len(filtered) == 1:
            co2 = int(filtered[0], base=2)
            break

    return oxygen * co2


if __name__ == '__main__':
    print(part_one())
    print(part_two())
