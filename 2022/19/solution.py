import collections
import copy
import re
import time

Cost = collections.namedtuple('Cost', ['ore', 'clay', 'obsidian'],
                              defaults=[0, 0, 0])

Blueprint = collections.namedtuple('Blueprint',
                                   ['id_number',
                                    'costs',
                                    'max_expenditure'])

def parse_blueprints(filename):
    with open(filename) as f:
        bps = list()
        lines = f.read().splitlines()
        for line in lines:
            xs = list(map(int, re.findall(r'\d+', line)))
            costs = {
                    "ore": Cost(ore=xs[1]),
                    "clay": Cost(ore=xs[2]),
                    "obsidian": Cost(ore=xs[3],clay=xs[4]),
                    "geode": Cost(ore=xs[5],obsidian=xs[6])
                    }
            max_expenditure = {
                    "ore": max((c.ore for c in costs.values())),
                    "clay": max((c.clay for c in costs.values())),
                    "obsidian": costs["geode"].obsidian
                    }
            bp = Blueprint(id_number=xs[0], costs=costs,
                           max_expenditure=max_expenditure)
            bps.append(bp)
        return bps

Move = collections.namedtuple('Move', ['bot_type', 'cost', 'duration', 'reward'])

def ceildiv(a, b): # upside-down floor division is ceiling division
    return -(a // -b)

class State:
    def __init__(self, blueprint, time_remaining):
        self.blueprint = blueprint
        self.time_remaining = time_remaining
        # we start with no resources and one ore-collecting robot
        self.resources = { "clay": 0, "ore": 0, "obsidian": 0 }
        self.bots = { "clay": 0, "ore": 1, "obsidian": 0, "geode": 0 }
        self.geodes = 0

    def upper_bound(self):
        # a generous upper bound on geodes
        # imagine that we are somehow able to build a geode bot every remaining minute
        return self.geodes + sum(range(self.time_remaining))

    def to_move(self, bot_type):
        cost = self.blueprint.costs[bot_type]._asdict()
        # do we already have enough of this bot type?
        if bot_type != "geode":
            if self.blueprint.max_expenditure[bot_type] <= self.bots[bot_type]:
                return None
        # wait until we have enough resources to start building
        # resources are consumed when construction begins
        waiting_time = 0
        for r in cost:
            diff = cost[r] - self.resources[r]
            if not diff: continue
            if not self.bots[r]: return None
            waiting_time = max(waiting_time, ceildiv(diff, self.bots[r]))
        # it takes one minute to build a robot
        duration = waiting_time + 1
        if duration >= self.time_remaining:
            return None
        # how many geodes will this bot crack?
        reward = self.time_remaining - duration if bot_type == "geode" else 0
        return Move(bot_type, cost, duration, reward)

    def moves(self): # generate valid moves
        for bot_type in self.bots:
            move = self.to_move(bot_type)
            if move is not None:
                yield move

    def apply(self, move):
        next_state = copy.deepcopy(self)
        next_state.time_remaining -= move.duration
        for r in next_state.resources:
            next_state.resources[r] += next_state.bots[r] * move.duration
            next_state.resources[r] -= move.cost[r]
        next_state.bots[move.bot_type] += 1
        next_state.geodes += move.reward
        return next_state

def visit(state, depth=0):
    best = state.geodes
    for move in state.moves():
        next_state = state.apply(move)
        if next_state.upper_bound() <= best:
            continue
        best = max(best, visit(next_state, depth=depth+1))
    return best

def try_blueprint(bp, time_limit=24):
    '''
    find the largest number of geodes that can be opened
    within the timelimit, using the given blueprint
    '''
    state = State(bp, time_limit)
    _start = time.time()
    result = visit(state)
    _end = time.time()
    print(f"time for Blueprint {bp.id_number}: {_end - _start}")
    return result

blueprints = parse_blueprints("input.txt")

max_geodes = list(map(try_blueprint, blueprints))
id_numbers = list(map(lambda bp: bp.id_number, blueprints))
quality_levels = [i * g for i, g in zip(max_geodes, id_numbers)]
print("Part 1:", sum(quality_levels))
print("- max geodes:", max_geodes)
print("- quality levels:", quality_levels)

max_geodes = list(map(lambda bp: try_blueprint(bp, time_limit=32), blueprints[:3]))
print("Part 2:", max_geodes[0] * max_geodes[1] * max_geodes[2])
print("- max geodes:", max_geodes)
