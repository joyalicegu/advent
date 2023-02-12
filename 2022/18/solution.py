def parse_voxels(filename):
    with open(filename) as f:
        return [tuple(map(int, line.strip().split(",")))
                for line in f.readlines()]

def neighbors(coordinates):
    return [tuple(x + c if i == j else x
                  for (i, x) in enumerate(coordinates))
            for j in range(len(coordinates))
            for c in (-1, 1)]

def unique_sides(voxels):
    sides = set()
    voxels = set(voxels)
    for voxel in voxels:
        for neighbor in neighbors(voxel):
            if neighbor not in voxels:
                side = tuple(sorted((voxel, neighbor)))
                sides.add(side)
    return sides

voxels = parse_voxels("input.txt")
sides = unique_sides(voxels)
print("Part 1:", len(sides))
