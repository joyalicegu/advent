def build_tree(commands):
    tree = {'/' : dict()}
    parents = []
    cwd = '/'
    cwt = tree['/']
    for command in commands:
        if not command: continue
        lines = command.splitlines()
        command = lines[0]
        output = lines[1:]
        if command.startswith('cd'):
            dst = command.split()[1]
            if dst == '/': # root
                parents = []
                cwd = '/'
                cwt = tree['/']
            elif dst == '..': # parent
                cwd, cwt = parents.pop()
            else: # child
                parents.append((cwd, cwt))
                cwd, cwt = dst, cwt[dst]
        elif command.startswith('ls'):
            for line in output:
                first, child = line.split()
                if child not in cwt:
                    if first == 'dir': # directory
                        cwt[child] = dict()
                    else: # file
                        cwt[child] = int(first)
    return tree

def total_sizes(tree):
    # sizes[0] will be the total size of the root directory
    sizes = [0]
    for v in tree.values():
        if isinstance(v, int):
            sizes[0] += v
        else:
            v_sizes = total_sizes(v)
            sizes[0] += v_sizes[0]
            sizes.extend(v_sizes)
    return sizes

with open("input.txt") as f:
    commands = f.read().split('$ ')

tree = build_tree(commands)
totals = total_sizes(tree['/'])
print("Part 1:", sum((total for total in totals if total <= 100000)))

used_space = totals[0]
unused_space = 70000000 - used_space
space_to_free = 30000000 - unused_space
print("Part 2:", min((total for total in totals if total >= space_to_free)))
