import itertools

def parse_numbers(filename):
    with open(filename) as f:
        return list(map(int, f.read().splitlines()))

def mix(numbers, decryption_key=None, repeats=1):
    if decryption_key is not None:
        numbers = list(map(lambda x: x * decryption_key, numbers))
    mixed = list(numbers)
    indices = list(range(len(numbers)))
    for i, n in itertools.islice(itertools.cycle(enumerate(numbers)), len(numbers) * repeats):
        old = indices.index(i)
        new = (old + n) % (len(numbers) - 1)
        mixed.insert(new, mixed.pop(old))
        indices.insert(new, indices.pop(old))
    return mixed

def extract(mixed):
    zero_index = mixed.index(0)
    return sum((mixed[(zero_index + j) % len(mixed)] for j in (1000, 2000, 3000)))

numbers = parse_numbers("input.txt")
print("Part 1:", extract(mix(numbers)))
print("Part 2:", extract(mix(numbers, decryption_key=811589153, repeats=10)))
