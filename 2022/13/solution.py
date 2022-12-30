import functools

with open("input.txt") as f:
    chunks = f.read().split('\n\n')
    pairs = [[eval(line) for line in chunk.splitlines()] for chunk in chunks]

def compare(left, right):
    '''
    return -1 if in the right order
    return 1 if not in the right order
    return 0 if continue checking the next part
    '''
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return -1
        elif left > right:
            return 1
        else:
            return 0
    elif isinstance(left, list) and isinstance(right, list):
        if not left and not right:
            return 0
        elif not left:
            return -1
        elif not right:
            return 1
        else:
            result = compare(left[0], right[0])
            if result:
                return result
            return compare(left[1:], right[1:])
    elif isinstance(left, list) and isinstance(right, int):
        return compare(left, [right])
    elif isinstance(left, int) and isinstance(right, list):
        return compare([left], right)
    return 0

indices = [i for i, pair in enumerate(pairs, start=1) if compare(pair[0], pair[1]) != 1]
print("Part 1:", sum(indices))

packets = [packet for pair in pairs for packet in pair]
packets.append([[2]])
packets.append([[6]])
packets.sort(key=functools.cmp_to_key(compare))
decoder_key = 1
for i, p in enumerate(packets, start=1):
    if compare(p, [[2]]) == 0 or compare(p, [[6]]) == 0:
        decoder_key *= i
print("Part 2:", decoder_key)
