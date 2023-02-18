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

def solve(monkeys, name):
    v = monkeys[name]
    if isinstance(v, int):
        return v
    elif isinstance(v, Operation):
        a = solve(monkeys, v.a)
        b = solve(monkeys, v.b)
        result = calculate(a, b, v.op)
        monkeys[name] = result
        return monkeys[name]

monkeys = parse_monkeys("input.txt")
print("Part 1:", solve(monkeys, "root"))
