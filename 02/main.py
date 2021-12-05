import os

with open(f"{os.path.dirname(__file__)}/input.txt") as f:
    INPUT = f.read()

array = [v.split() for v in INPUT.splitlines()]

def part_one():
    horiz = 0
    depth = 0

    for k, v in array:
        v = int(v)
        match k:
            case 'forward':
                horiz += v
            case 'up':
                depth -= v
            case 'down':
                depth += v

    return horiz * depth

def part_two():
    horiz = 0
    depth = 0
    aim = 0

    for k, v in array:
        v = int(v)
        match k:
            case 'forward':
                horiz += v
                depth += v * aim
            case 'up':
                aim -= v
            case 'down':
                aim += v

    return horiz * depth

if __name__ == '__main__':
    print(part_one())
    print(part_two())
