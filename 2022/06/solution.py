from itertools import combinations

def differ(args):
    return all((x ^ y for x, y in combinations(args, 2)))

def marker(datastream, window):
    shifted = [datastream[i: len(datastream) - window + 1 + i] for i in range(window)]
    differing = [differ(args) for args in zip(*shifted)]
    indices = [i + window for i, different in enumerate(differing) if different]
    return indices[0]

with open("input.txt", "rb") as f:
    datastream = f.read().strip()

test_datastream = b'mjqjpqmgbljsphdztnvjfqwrcgsmlb'
assert marker(test_datastream, 4) == 7
assert marker(test_datastream, 14) == 19

print(marker(datastream, 4))
print(marker(datastream, 14))
