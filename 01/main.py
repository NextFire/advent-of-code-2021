with open('input.txt', 'r') as f:
    input = f.read()

elves = ((int(i) for i in elf.split()) for elf in input.split('\n\n'))
sums = map(sum, elves)
sorted_sums = sorted(sums, reverse=True)

print(sorted_sums[0])
print(sum(sorted_sums[:3]))
