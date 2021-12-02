from input import INPUT

array = [v.split() for v in INPUT.splitlines()]

horiz = 0
depth = 0

for k, v in array:
    v = int(v)
    if k == 'forward':
        horiz += v
    elif k == 'up':
        depth -= v
    elif k == 'down':
        depth += v

print(horiz * depth)

horiz = 0
depth = 0
aim = 0

for k, v in array:
    v = int(v)
    if k == 'forward':
        horiz += v
        depth += v * aim
    elif k == 'up':
        aim -= v
    elif k == 'down':
        aim += v

print(horiz * depth)
