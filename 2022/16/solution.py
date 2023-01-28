# blatantly plagiarized from:
# - https://fasterthanli.me/series/advent-of-code-2022/part-16
# - https://www.reddit.com/r/adventofcode/comments/zn6k1l/comment/j2xhog7/

import itertools
import math
import re
import copy

class Valve:
    def __init__(self, flow, neighbors):
        self.flow = flow
        self.neighbors = neighbors

def parse_valves(filename):
    PATTERN = r"Valve (?P<name>\S+) has flow rate=(?P<flow>\d+);" \
            r" tunnel(s?) lead(s?) to valve(s?) (?P<neighbors>.*)"
    scan = dict()
    with open(filename) as f:
        for line in f.read().splitlines():
            match = re.match(PATTERN, line)
            assert match is not None
            name = match.group("name")
            flow = int(match.group("flow"))
            neighbors = match.group("neighbors").split(", ")
            scan[name] = Valve(flow, neighbors)
    return scan

VALVES = parse_valves("input.txt")
FLOWS = { name : v.flow for name, v in VALVES.items() if v.flow > 0 }

# arbitrary name-to-index mapping for open-valves bitmask
INDICES = { name : 1 << i for i, name in enumerate(VALVES) }

# floyd-warshall for shortest distance between any two valves
# https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
DISTANCES = { (i, j) : 1 if i in VALVES[j].neighbors else math.inf
             for i in VALVES for j in VALVES}
for k, i, j in itertools.permutations(VALVES, 3):
    DISTANCES[i, j] = min(DISTANCES[i, j], DISTANCES[i, k] + DISTANCES[k, j])

class Move:
    def __init__(self, target, cost, reward):
        self.target = target
        self.cost = cost
        self.reward = reward

class State:
    def __init__(self, position='AA', time_remaining=30):
        self.position = position
        self.time_remaining = time_remaining
        self.pressure = 0
        self.open_valves = 0 # bitmask

    def moves(self):
        def to_move(target, flow):
            cost = DISTANCES[self.position, target] + 1
            minutes_open = self.time_remaining - cost
            if minutes_open <= 0 or INDICES[target] & self.open_valves:
                return None
            reward = flow * minutes_open
            return Move(target, cost, reward)
        return list(filter(lambda move: move is not None,
                (to_move(target, flow) for target, flow in FLOWS.items())))

    def apply(self, move):
        next_state = copy.copy(self)
        next_state.position = move.target
        next_state.time_remaining -= move.cost
        next_state.pressure += move.reward
        next_state.open_valves |= INDICES[move.target]
        return next_state

# build an open-valves to best-pressure mapping
def visit(state, visited=None):
    if visited is None: visited = dict()
    visited[state.open_valves] = max(visited.get(state.open_valves, 0), state.pressure)
    for move in state.moves():
        next_state = state.apply(move)
        visit(next_state, visited)
    return visited

visited = visit(State())
print("Part 1:", max(visited.values()))

visited = visit(State(time_remaining=26))
disjoint_sums = (v1 + v2 for ov1, v1 in visited.items() for ov2, v2 in visited.items() if ov1 & ov2 == 0)
print("Part 2:", max(disjoint_sums))
