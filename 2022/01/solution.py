totals = [0]
with open('input.txt') as f:
    # more complicated than it needs to be - should have done f.read().split('\n\n')
    lines = [line.rstrip() for line in f]
    empty = [i for (i, line) in enumerate(lines) if not line]
    starts, ends = [None] + [i + 1 for i in empty], empty + [None]
    totals = [sum(map(int, lines[i:j])) for (i, j) in zip(starts, ends)]
print("Part 1:", max(totals))
print("Part 2:", sum(sorted(totals)[-3:]))
