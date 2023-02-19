import collections
import enum
import re

FACING_STRINGS = { 0 : '>', 1 : 'v', 2 : '<', 3 : '^' }

class Facing(enum.IntEnum):
    RIGHT = 0
    DOWN = 1
    LEFT = 2
    UP = 3

    def cw(self):
        return Facing((self + 1) % 4)

    def ccw(self):
        return Facing((self - 1) % 4)

    def opposite(self):
        return Facing((self + 2) % 4)

    def __str__(self):
        return FACING_STRINGS[self]

Position = collections.namedtuple('Position', ['row', 'col', 'facing'])

def pad_board(board):
    width = max((len(row) for row in board))
    for row in board:
        row.extend([' '] * (width - len(row)))
    return board

def off_map(board, i, j):
    return i < 0 or j < 0 or i >= len(board) or j >= len(board[i]) or board[i][j] == ' '

def unchecked_move(i, j, direction):
    if direction == Facing.RIGHT:
        j += 1
    elif direction == Facing.DOWN:
        i += 1
    elif direction == Facing.LEFT:
        j -= 1
    elif direction == Facing.UP:
        i -= 1
    return i, j

def is_boundary_position(board, position):
    i, j = position.row - 1, position.col - 1
    if off_map(board, i, j):
        return False
    i, j = unchecked_move(i, j, position.facing)
    return off_map(board, i, j)

def next_boundary_position(board, position):
    # going around the boundary clockwise
    # try moving along the same direction
    r, c = unchecked_move(position.row, position.col, position.facing.cw())
    position_ahead = Position(r, c, position.facing)
    if is_boundary_position(board, position_ahead):
        return position_ahead
    # try turning counterclockwise (concave corner)
    r, c = unchecked_move(position_ahead.row, position_ahead.col, position_ahead.facing)
    position_ccw = Position(r, c, position.facing.ccw())
    if is_boundary_position(board, position_ccw):
        return position_ccw
    # try turning clockwise (convex corner)
    position_cw = Position(position.row, position.col, position.facing.cw())
    if is_boundary_position(board, position_cw):
        return position_cw
    assert False # shouldn't happen

def build_cube_connections(board):
    connections = dict() # position to position mapping for boundary positions
    boundary = list()
    pairs = collections.deque()
    # 1. trace the boundary
    # starting from the top left corner
    col = next(i for i, tile in enumerate(board[0]) if tile != ' ') + 1
    position = Position(row=1, col=col, facing=Facing.UP)
    while not boundary or position != boundary[0]:
        boundary.append(position)
        prev_position = position
        position = next_boundary_position(board, position)
        if prev_position.facing.ccw() == position.facing: # concave corner
            pairs.append((len(boundary) - 1, len(boundary)))
    # 2. search outwards from the concave corners and connect pairs
    while pairs:
        i, j = pairs.popleft()
        # stitch this pair together
        connections[boundary[i]] = boundary[j]._replace(facing=boundary[j].facing.opposite())
        connections[boundary[j]] = boundary[i]._replace(facing=boundary[i].facing.opposite())
        # add next pair to queue (if not already visited)
        ni = (i - 1) % len(boundary)
        nj = (j + 1) % len(boundary)
        # pair can't be on the same edge if we've turned in opposite directions
        iturn = boundary[ni].facing - boundary[i].facing
        jturn = boundary[nj].facing - boundary[j].facing
        if iturn and jturn and iturn == -jturn:
            continue
        # if not already visited
        if boundary[ni] not in connections and boundary[nj] not in connections:
            pairs.append((ni, nj))
    return connections

def parse_notes(filename):
    with open(filename) as f:
        board_str, path_str = f.read().split("\n\n")
    board = pad_board(list(list(line) for line in board_str.splitlines()))
    path = [int(x) if x and x.isnumeric() else x for x in re.findall(r"(\d+|L|R)", path_str)]
    # initial position is leftmost open tiles facing right
    col = next(i for i, tile in enumerate(board[0]) if tile == '.') + 1 # 1-indexing
    position = Position(row=1, col=col, facing=Facing.RIGHT)
    connections = build_cube_connections(board)
    return board, connections, path, position

def forward_one(board, connections, position):
    # return position after moving forward one tile (with toroidal wrapping)
    if position in connections:
        new_position = connections[position]
        i, j = new_position.row - 1, new_position.col - 1
        tile = board[i][j]
    else: # fallback to toroidal
        i = position.row - 1
        j = position.col - 1
        tile = ' '
        while tile == ' ':
            if position.facing == Facing.RIGHT:
                j = (j + 1) % len(board[i])
            elif position.facing == Facing.DOWN:
                i = (i + 1) % len(board)
            elif position.facing == Facing.LEFT:
                j = (j - 1) % len(board[i])
            elif position.facing == Facing.UP:
                i = (i - 1) % len(board)
            tile = board[i][j]
        new_position = position._replace(row=i+1, col=j+1)
    if tile == '#':
        return None
    return new_position

def turn(direction, position):
    new_facing = position.facing.cw() if direction == 'R' else position.facing.ccw()
    return position._replace(facing=new_facing)

def print_board(board): # debug
    print("\n".join(("".join(row) for row in board)))

def follow_path(board, connections, path, position, debug=False):
    if debug:
        board[position.row-1][position.col-1] = str(position.facing)
    for n in path:
        if isinstance(n, int):
            for _ in range(n):
                next_position = forward_one(board, connections, position)
                if next_position == None: # hit wall
                    break
                position = next_position
                if debug:
                    board[position.row-1][position.col-1] = str(position.facing)
        else:
            position = turn(n, position)
            if debug:
                board[position.row-1][position.col-1] = str(position.facing)
    return position

def calculate_password(position):
    return 1000 * position.row + 4 * position.col + position.facing

board, connections, path, start = parse_notes("input.txt")
position = follow_path(board, dict(), path, start)
password = calculate_password(position)
print("Part 1:", password)
position = follow_path(board, connections, path, start)
password = calculate_password(position)
print("Part 2:", password)
