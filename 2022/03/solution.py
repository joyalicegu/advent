def priority(item):
    if item.isupper():
        return ord(item) - ord('A') + 27
    return ord(item) - ord('a') + 1

with open("input.txt") as f:
    rucksacks = [l.rstrip() for l in f.readlines()]

# part 1
total = 0
for rucksack in rucksacks:
    half = len(rucksack) // 2
    first = rucksack[:half]
    second = rucksack[half:]
    item = set(first).intersection(set(second)).pop()
    total += priority(item)
print(total)

# part 2
total = 0
for i in range(0, len(rucksacks), 3):
    a, b, c = rucksacks[i], rucksacks[i+1], rucksacks[i+2]
    badge = set(a).intersection(set(b)).intersection(set(c)).pop()
    total += priority(badge)
print(total)
