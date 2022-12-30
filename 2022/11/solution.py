from collections import deque
from functools import reduce
import copy
import operator

def parse_monkey(paragraph):
    monkey = dict()
    lines = paragraph.splitlines()
    monkey['items'] = deque(map(int, lines[1].split(': ')[1].split(', ')))
    monkey['operation'] = lines[2].split('= ')[1]
    monkey['test'] = int(lines[3].split()[-1])
    monkey[True] = int(lines[4].split()[-1])
    monkey[False] = int(lines[5].split()[-1])
    return monkey

def inspect(old, operation):
    return eval(operation)

def simulate(rounds, monkeys, part=1):
    activity = [0 for _ in monkeys]
    MAX_WORRY = reduce(operator.mul, (monkey['test'] for monkey in monkeys))
    for _ in range(rounds):
        for i in range(len(monkeys)):
            monkey = monkeys[i]
            while monkey['items']:
                item = monkey['items'].popleft()
                item = inspect(item, monkey['operation'])
                if part == 1:
                    item //= 3
                if part == 2:
                    item %= MAX_WORRY
                activity[i] += 1
                j = monkey[item % monkey['test'] == 0]
                monkeys[j]['items'].append(item)
    return activity

with open("input.txt") as f:
    monkeys = [parse_monkey(paragraph)
               for paragraph
               in f.read().split("\n\n")]

activity = simulate(20, copy.deepcopy(monkeys), part=1)
print("Part 1:", reduce(operator.mul, sorted(activity, reverse=True)[:2]))
activity = simulate(10000, monkeys, part=2)
print("Part 2:", reduce(operator.mul, sorted(activity, reverse=True)[:2]))
