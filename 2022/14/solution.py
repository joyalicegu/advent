SAND_SOURCE = 500, 0

def print_grid(grid, min_x, max_x):
    for d in range(3, 0, -1):
        print("   ", " ".join((str(j)[-d]
                             for j in range(min_x, max_x + 1))))
    for i, row in enumerate(grid):
        print(f'{i:3}', " ".join(row))
    return

def pour_sand(grid, min_x, max_x, min_y, max_y, has_floor=False):
    sand_x, sand_y = SAND_SOURCE
    while min_x <= sand_x <= max_x and min_y <= sand_y <= max_y:
        if has_floor and sand_y == max_y: # resting on the floor
            return (sand_x, sand_y)
        down_x, down_y = sand_x, sand_y + 1
        left_x, left_y = sand_x - 1, sand_y + 1
        right_x, right_y = sand_x + 1, sand_y + 1
        try:
            if grid[down_y - min_y][down_x - min_x] == '.':
                sand_x, sand_y = down_x, down_y
            elif grid[left_y - min_y][left_x - min_x] == '.':
                sand_x, sand_y = left_x, left_y
            elif grid[right_y - min_y][right_x - min_x] == '.':
                sand_x, sand_y = right_x, right_y
            else:
                return (sand_x, sand_y) # resting position
        except IndexError:
            break
    return None # sand fell into the abyss

def simulate_sand(paths, has_floor=False):
    max_x = SAND_SOURCE[0]
    max_y = 0
    min_x = SAND_SOURCE[0]
    min_y = 0 # we know it will be 0
    for path in paths:
        for (x, y) in path:
            max_x = max(x, max_x)
            max_y = max(y, max_y)
            min_x = min(x, min_x)

    if has_floor: # widen grid dimensions
        max_y += 1
        min_x = SAND_SOURCE[0] - max_y
        max_x = SAND_SOURCE[0] + max_y

    grid = [['.' for _ in range(min_x, max_x + 1)] for _ in range(max_y + 1)]

    # populate grid with rock
    for path in paths:
        for segment in zip(path[:-1], path[1:]):
            (x1, y1), (x2, y2) = segment
            for y in range(min(y1, y2), max(y1, y2) + 1): # if y varies
                grid[y - min_y][x1 - min_x] = '#'
            for x in range(min(x1, x2), max(x1, x2) + 1): # if x varies
                grid[y1 - min_y][x - min_x] = '#'

    # pour sand until sand falls into the abyss
    units = 0
    while True:
        resting = pour_sand(grid, min_x, max_x, min_y, max_y, has_floor=has_floor)
        if resting is None:
            break
        sand_x, sand_y = resting
        grid[sand_y - min_y][sand_x - min_x] = 'o' # draw sand on grid
        units += 1
        if resting == SAND_SOURCE:
            break
    return units

with open("input.txt") as f:
    lines = f.read().splitlines()
    paths = [[tuple(map(int, pair.split(',')))
              for pair in line.split(' -> ')]
             for line in lines]

print("Part 1:", simulate_sand(paths))
print("Part 2:", simulate_sand(paths, has_floor=True))
