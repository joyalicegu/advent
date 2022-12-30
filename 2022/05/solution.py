moves = []
stacks = []

'''
Convert
    "    [D]    "
    "[N] [C]    "
    "[Z] [M] [P]"
    " 1   2   3 "
into
stacks = [
    [], # dummy stack for ease of indexing
    ['Z', 'N'],
    ['M', 'C', 'D'],
    ['P']
    ]
'''
def parse_stacks(a):
    lines = a.split('\n')
    rotated_stacks = [list(line)[1::4] for line in lines[:-1]]
    stacks = [[c for c in stack if c != ' '] for stack in zip(*rotated_stacks[::-1])]
    stacks = [[]] + stacks # prepend a dummy stack for indexing purposes
    return stacks

with open("input.txt") as f:
    top, bottom = f.read().split('\n\n')
    stacks_one = parse_stacks(top)
    stacks_two = parse_stacks(top)
    tokens = [line.split() for line in bottom.split('\n') if line]
    moves = [(int(t[1]), int(t[3]), int(t[5])) for t in tokens]

for move in moves:
    count, src, dst = move
    # move count crates from src to dst
    # part 1:
    for _ in range(count):
        crate = stacks_one[src].pop()
        stacks_one[dst].append(crate)
    # part 2:
    crates = stacks_two[src][-count:]
    stacks_two[src] = stacks_two[src][:-count]
    stacks_two[dst].extend(crates)

# which crate will end up on top of each stack?
print("".join([stack[-1] for stack in stacks_one[1:]]))
print("".join([stack[-1] for stack in stacks_two[1:]]))
