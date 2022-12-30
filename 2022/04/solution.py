def parse_line(line):
    [ab, cd] = line.rstrip().split(",")
    [a, b] = map(int, ab.split('-'))
    [c, d] = map(int, cd.split('-'))
    return (a, b, c, d)

# true if and only if pair1 contains pair2
def contains(pair1, pair2):
    (a, b), (c, d) = pair1, pair2
    return (a <= c) and (b >= d)

def overlap(pair1, pair2):
    (a, b), (c, d) = pair1, pair2
    return (b >= c) and (d >= a)

contains_count = 0
overlap_count = 0
with open("input.txt") as f:
    for line in f.readlines():
        (a, b, c, d) = parse_line(line)
        if contains((a, b), (c, d)) or contains((c, d), (a, b)):
            contains_count += 1
        if overlap((a, b), (c, d)):
            overlap_count += 1
print(contains_count)
print(overlap_count)
