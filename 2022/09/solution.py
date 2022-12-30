with open("input.txt") as f:
    lines = f.read().splitlines()

DIRECTIONS = {'U' : (0, 1), 'R' : (1, 0), 'D' : (0, -1), 'L' : (-1, 0)}

motions = [(DIRECTIONS[line[0]], int(line.split()[1])) for line in lines]

def move_head(head, direction):
    dx, dy = direction
    x, y = head
    return (x + dx, y + dy)

def follow_head(head, tail):
    x, y = tail
    head_x, head_y = head
    dx, dy = head_x - x, head_y - y
    if dy == 0 and abs(dx) > 1:
        x += dx // abs(dx)
    elif dx == 0 and abs(dy) > 1:
        y += dy // abs(dy)
    elif abs(dy) > 1 or abs(dx) > 1:
        x += dx // abs(dx)
        y += dy // abs(dy)
    return (x, y)

def simulate_rope(motions, length):
    knots = [(0, 0) for _ in range(length)]
    visited = {knots[-1]}
    for (direction, steps) in motions:
        for _ in range(steps):
            knots[0] = move_head(knots[0], direction)
            for i in range(1, length):
                knots[i] = follow_head(knots[i - 1], knots[i])
            visited.add(knots[-1])
    return len(visited)

print("Part 1:", simulate_rope(motions, 2))
print("Part 2:", simulate_rope(motions, 10))
