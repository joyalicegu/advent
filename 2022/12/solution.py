import collections

DIRECTIONS = [(0, 1), (0, -1), (1, 0), (-1, 0)]

def elevation(ch):
    if ch == 'S':
        return ord('a')
    if ch == 'E':
        return ord('z')
    return ord(ch)

def traverse(heightmap, start, end, part2=False):
    rows = len(heightmap)
    cols = len(heightmap[0])
    queue = collections.deque([(start, 0, [])])
    visited = set([start])
    while queue:
        (r, c), steps, path = queue.popleft()
        assert steps == len(path)
        if (r, c) == end or (part2 and elevation(heightmap[r][c]) == ord('a')):
            return steps, path
        steps += 1
        for nr, nc in ((r + dr, c + dc) for dr, dc in DIRECTIONS):
            if 0 <= nr < rows and 0 <= nc < cols and (nr, nc) not in visited:
                height = elevation(heightmap[r][c])
                next_height = elevation(heightmap[nr][nc])
                if part2:
                    next_height, height = height, next_height
                if next_height - height <= 1:
                    queue.append(((nr, nc), steps, path + [(nr, nc)]))
                    visited.add((nr, nc))
    return None

with open('input.txt') as f:
    heightmap = f.read().splitlines()

start = next(((i, line.find('S')) for i, line in enumerate(heightmap) if 'S' in line))
end = next(((i, line.find('E')) for i, line in enumerate(heightmap) if 'E' in line))

steps, path = traverse(heightmap, start, end)
print("Part 1:", steps)

steps, path = traverse(heightmap, end, start, part2=True)
print("Part 2:", steps)
