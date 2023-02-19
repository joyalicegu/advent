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

    def __str__(self):
        return FACING_STRINGS[self]

Position = collections.namedtuple('Position', ['row', 'col', 'facing'])

def pad_board(board):
    width = max((len(row) for row in board))
    for row in board:
        row.extend([' '] * (width - len(row)))
    return board

def parse_notes(filename):
    with open(filename) as f:
        board_str, path_str = f.read().split("\n\n")
    board = pad_board(list(list(line) for line in board_str.splitlines()))
    path = [int(x) if x and x.isnumeric() else x for x in re.findall(r"(\d+|L|R)", path_str)]
    # initial position is leftmost open tiles facing right
    col = next(i for i, tile in enumerate(board[0]) if tile == '.') + 1 # 1-indexing
    board[0][col-1] = '>' # debug
    position = Position(row=1, col=col, facing=Facing.RIGHT)
    return board, path, position

def forward_one_toroidal(board, position):
    # return position after moving forward one tile (with toroidal wrapping)
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
    if tile == '#':
        return None
    return position._replace(row=i+1, col=j+1)

def turn(direction, position):
    new_facing = position.facing.cw() if direction == 'R' else position.facing.ccw()
    return position._replace(facing=new_facing)

def print_board(board): # debug
    print("\n".join(("".join(row) for row in board)))

def follow_path(board, path, position):
    for n in path:
        if isinstance(n, int):
            for _ in range(n):
                next_position = forward_one_toroidal(board, position)
                if next_position == None: # hit wall
                    break
                position = next_position
                board[position.row-1][position.col-1] = str(position.facing) # debug
        else:
            position = turn(n, position)
            board[position.row-1][position.col-1] = str(position.facing) # debug
    return position

def calculate_password(position):
    return 1000 * position.row + 4 * position.col + position.facing

board, path, position = parse_notes("input.txt")
position = follow_path(board, path, position)
print_board(board) # debug
password = calculate_password(position)
print("Part 1:", password)
