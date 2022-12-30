# test
lines = """30373
25512
65332
33549
35390""".splitlines()

with open("input.txt") as f:
    lines = f.read().splitlines()

grid = [[int(c) for c in line] for line in lines]
rows, cols = len(grid), len(grid[0])

def print_grid(grid): # delete later
    for i in range(len(grid)):
        print(grid[i])

""" Part 1 """

# from left
from_left = [[False for _ in range(cols)] for _ in range(rows)]
for r in range(rows):
    max_height = 0
    for c in range(cols):
        if c == 0 or grid[r][c] > max_height:
            from_left[r][c] = True
        max_height = max(max_height, grid[r][c])

# from right
from_right = [[False for _ in range(cols)] for _ in range(rows)]
for r in range(rows):
    max_height = 0
    for c in reversed(range(cols)):
        if c == cols - 1 or grid[r][c] > max_height:
            from_right[r][c] = True
        max_height = max(max_height, grid[r][c])

# from top
from_top = [[False for _ in range(cols)] for _ in range(rows)]
for c in range(cols):
    max_height = 0
    for r in range(rows):
        if r == 0 or grid[r][c] > max_height:
            from_top[r][c] = True
        max_height = max(max_height, grid[r][c])

# from bottom
from_bottom = [[False for _ in range(cols)] for _ in range(rows)]
for c in range(cols):
    max_height = 0
    for r in reversed(range(rows)):
        if r == rows - 1 or grid[r][c] > max_height:
            from_bottom[r][c] = True
        max_height = max(max_height, grid[r][c])

# How many trees are visible from outside the grid?
from_any = [[any((from_left[r][c], from_right[r][c], from_top[r][c], from_bottom[r][c])) for c in range(cols)] for r in range(rows)]
print("Part 1:", sum(sum(map(int, row)) for row in from_any))

""" Part 2 """

# What is the highest scenic score possible for any tree?
# (just do the obvious thing)
def distance(grid, position, direction):
    heights = []
    r, c = position
    dr, dc = direction
    while 0 <= r < len(grid) and 0 <= c < len(grid[0]):
        heights.append(grid[r][c])
        r, c = r + dr, c + dc
    total = 0
    r, c = position
    our_height = grid[r][c]
    for height in heights[1:]:
        total += 1
        if height >= our_height:
            break
    return total

from math import prod
DIRECTIONS = [(0, 1), (0, -1), (-1, 0), (1, 0)]
def score(grid, position):
    return prod((distance(grid, position, d) for d in DIRECTIONS))

scores = [[score(grid, (r, c)) for c in range(cols)] for r in range(rows)]
print("Part 2:", max(map(max, scores)))
