with open("input.txt") as f:
    instructions = [line.split() for line in f.readlines()]

cycles = 0
x = 1
xs = [0] # xs[i] - value of x during cycle i

for i in instructions:
    if i[0] == 'addx':
        cycles += 1
        xs.append(x)
        cycles += 1
        xs.append(x)
        x += int(i[1])
    elif i[0] == 'noop':
        cycles += 1
        xs.append(x)

assert len(xs) == cycles + 1
signal_strengths = [xs[i] * i for i in range(20, cycles + 1, 40)]
print("Part 1:", sum(signal_strengths))

screen = ""
for cycle, x in enumerate(xs[1:], start=1):
    column = (cycle - 1) % 40
    if x - 1 <= column <= x + 1:
        screen += '#'
    else:
        screen += ' '
    if column == 39:
        screen += '\n'
print("Part 2:")
print(screen)
