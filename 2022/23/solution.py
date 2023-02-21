def parse_elves(filename):
    with open(filename) as f:
        elves = {(r, c)
                 for r, line in enumerate(f.read().splitlines())
                 for c, tile in enumerate(line) if tile == '#'}
        return elves

DIRECTIONS = ['N', 'S', 'W', 'E']

ASSOCIATED_DIRECTIONS = {
        'N': ('N','NE','NW'),
        'S': ('S','SE','SW'),
        'W': ('W','NW','SW'),
        'E': ('E','NE','SE'),
        }

MOVE = {
        'N' : (lambda r, c: (r-1, c  )),
        'NE': (lambda r, c: (r-1, c+1)),
        'E' : (lambda r, c: (r  , c+1)),
        'SE': (lambda r, c: (r+1, c+1)),
        'S' : (lambda r, c: (r+1, c  )),
        'SW': (lambda r, c: (r+1, c-1)),
        'W' : (lambda r, c: (r  , c-1)),
        'NW': (lambda r, c: (r-1, c-1)),
        }

def neighbors(elf):
    return [MOVE[d](*elf) for d in MOVE]

def neighbors_in_direction(elf, direction):
    return [MOVE[d](*elf) for d in ASSOCIATED_DIRECTIONS[direction]]

def elf_neighbors(elf, elves):
    return [neighbor
            for neighbor in neighbors(elf)
            if neighbor in elves]

def elf_neighbors_in_direction(elf, elves, direction):
    return [neighbor
            for neighbor in neighbors_in_direction(elf, direction)
            if neighbor in elves]

def directions_for_round(current_round):
    i = (current_round - 1) % len(DIRECTIONS)
    return tuple(DIRECTIONS[i:len(DIRECTIONS)] + DIRECTIONS[0:i])

def do_round(elves, current_round):
    done = False
    # first half of round
    proposals = dict()
    for elf in elves:
        # if there are no adjacent elves, the elf does nothing
        if not elf_neighbors(elf, elves):
            continue
        # consider each direction
        for d in directions_for_round(current_round):
            if not elf_neighbors_in_direction(elf, elves, d):
                proposal = MOVE[d](*elf)
                if proposal not in proposals:
                    proposals[proposal] = []
                proposals[proposal].append(elf)
                break
    # second half of round
    # a proposal is valid if there is only one contender
    for proposal in list(proposals):
        contenders = proposals[proposal]
        if len(contenders) == 1:
            proposals[proposal] = contenders[0]
        else:
            proposals.pop(proposal, None)
    # move elves
    done = not proposals
    for proposal, elf in proposals.items():
        if elf not in proposals:
            elves.remove(elf)
        if proposal not in elves:
            elves.add(proposal)
    return elves, done

def do_rounds(elves, start=1, rounds=None, debug=False):
    if debug:
        elf_count = len(elves)
    current_round = start
    final = None if rounds is None else start + rounds - 1
    done = False
    while not done and (rounds is None or current_round <= final):
        elves, done = do_round(elves, current_round)
        if debug:
            print(f"end of round {current_round} - score {score(elves)}")
            assert len(elves) == elf_count
        current_round += 1
    return elves, current_round - 1

def bounding_box(elves): # inclusive-ends
    row_min = min((r for (r, c) in elves))
    row_max = max((r for (r, c) in elves))
    col_min = min((c for (r, c) in elves))
    col_max = max((c for (r, c) in elves))
    return row_min, row_max, col_min, col_max

def print_elves(elves):
    row_min, row_max, col_min, col_max = bounding_box(elves)
    s = ""
    for r in range(row_min, row_max + 1):
        for c in range(row_min, row_max + 1):
            s += "#" if (r, c) in elves else '.'
        s += "\n"
    print(s)

def score(elves):
    row_min, row_max, col_min, col_max = bounding_box(elves)
    return (row_max - row_min + 1) * (col_max - col_min + 1) - len(elves)

elves = parse_elves("input.txt")
elves, final_round = do_rounds(elves, rounds=10)
print(f"Part 1:", score(elves))
elves, final_round = do_rounds(elves, start=11)
print(f"Part 2:", final_round)
