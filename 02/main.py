from input import INPUT

array = [v.split() for v in INPUT.splitlines()]

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

print(horiz * depth)

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

print(horiz * depth)
