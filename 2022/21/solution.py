import collections

Operation = collections.namedtuple('Operation', ['a', 'b', 'op'])

def parse_monkeys(filename):
    monkeys = dict()
    with open(filename) as f:
        for line in f.readlines():
            name, etc = line.split(':')
            try:
                monkeys[name] = int(etc)
            except ValueError:
                a, op, b = etc.split()
                monkeys[name] = Operation(a=a, b=b, op=op)
    return monkeys

def calculate(a, b, op):
    if op == '+':
        return a + b
    elif op == '-':
        return a - b
    elif op == '*':
        return a * b
    elif op == '/':
        return a // b

def solve(monkeys, name, unknown=None):
    v = monkeys[name]
    if name == unknown:
        return None
    if isinstance(v, int):
        return v
    elif isinstance(v, Operation):
        a = solve(monkeys, v.a, unknown=unknown)
        if a is None: return None
        b = solve(monkeys, v.b, unknown=unknown)
        if b is None: return None
        result = calculate(a, b, v.op)
        monkeys[name] = result
        return monkeys[name]

def uncalculate(a, b, op, target):
    if op == '=':
        if a is None: return b
        if b is None: return a
    elif op == '+':
        if a is None: return target - b
        if b is None: return target - a
    elif op == '-':
        if a is None: return target + b
        if b is None: return a - target
    elif op == '*':
        if a is None: return target // b
        if b is None: return target // a
    elif op == '/':
        if a is None: return target * b
        if b is None: return a // target

def backsolve(monkeys, name, unknown, target=None):
    if name == unknown: return target
    v = monkeys[name]
    a = solve(monkeys, v.a, unknown=unknown)
    b = solve(monkeys, v.b, unknown=unknown)
    op = '=' if name == "root" else v.op
    target = uncalculate(a, b, op, target)
    name = v.a if a is None else v.b
    return backsolve(monkeys, name, unknown, target)

monkeys = parse_monkeys("input.txt")
print("Part 1:", solve(monkeys, "root"))

monkeys = parse_monkeys("input.txt")
print("Part 2:", backsolve(monkeys, "root", "humn"))
