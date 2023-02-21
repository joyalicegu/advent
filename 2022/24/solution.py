import collections

DIRECTIONS = "^>v<"

MOVE = {
        '^' : (lambda r, c: (r-1, c  )),
        '>' : (lambda r, c: (r  , c+1)),
        'v' : (lambda r, c: (r+1, c  )),
        '<' : (lambda r, c: (r  , c-1)),
        }

OPPOSITE = {
        '^' : 'v',
        '>' : '<',
        'v' : '^',
        '<' : '>',
        }


def parse_valley(filename):
    with open(filename) as f:
        lines = f.read().splitlines()
        rows = len(lines)
        cols = len(lines[0])
        blizzards = {((r, c), d)
                     for r, line in enumerate(lines)
                     for c, d in enumerate(line)
                     if d in DIRECTIONS}
        return rows, cols, blizzards

def is_out_of_bounds(position, rows, cols, start, goal):
    if position == start or position == goal:
        return False
    r, c = position
    return r <= 0 or c <= 0 or r >= rows - 1 or c >= cols - 1

def blizzard_start(rows, cols, position, direction, time):
    # start position of blizzard at position with direction at time
    r, c = position
    opposite = OPPOSITE[direction] # flip direction
    if opposite == '^':
        r -= 1; r -= time; r %= rows - 2; r += 1
    if opposite == '>':
        c -= 1; c += time; c %= cols - 2; c += 1
    if opposite == 'v':
        r -= 1; r += time; r %= rows - 2; r += 1
    if opposite == '<':
        c -= 1; c -= time; c %= cols - 2; c += 1
    return r, c

def is_blizzard(position, rows, cols, blizzards, time):
    for direction in DIRECTIONS:
        start = blizzard_start(rows, cols, position, direction, time)
        if (start, direction) in blizzards: # blizzard exists
            return True
    return False

def search(rows, cols, blizzards, start=None, goal=None, time=0):
    starting_time = time
    visited = set()
    max_time = time # progress indicator
    queue = collections.deque([(start, starting_time)])
    while queue:
        position, time = queue.popleft()
        if (position, time) in visited:
            continue
        visited.add((position, time))
        if position == goal:
            print(f"\nReached {goal} from {start} at minute {time} in {time - starting_time} minutes")
            return time
        if time > max_time:
            print(".", end='', flush=True) # progress indicator
            max_time = time
        next_time = time + 1
        moves = [MOVE[direction](*position) for direction in DIRECTIONS]
        moves.append(position) # wait
        moves = [m for m in moves if not is_out_of_bounds(m, rows, cols, start, goal)]
        for move in moves:
            # if move == goal: # TODO delete this
            #     print(move, "is goal (move)")
            #     return next_time
            if is_blizzard(move, rows, cols, blizzards, next_time):
                continue
            else:
                queue.append((move, next_time))
    return None

rows, cols, blizzards = parse_valley("input.txt")
start = (0, 1) # 1 right of upper left corner
goal = (rows - 1, cols - 2) # 1 left of lower right corner
time = search(rows, cols, blizzards, start=start, goal=goal)
print("Part 1:", time)
time = search(rows, cols, blizzards, start=goal, goal=start, time=time)
time = search(rows, cols, blizzards, start=start, goal=goal, time=time)
print("Part 2:", time)
