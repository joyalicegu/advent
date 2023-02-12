import collections
import itertools
import operator

Side = collections.namedtuple('Side', ['inside', 'outside'])
Edge = collections.namedtuple('Edge', ['inside', 'outside'])

def parse_voxels(filename):
    with open(filename) as f:
        return [tuple(map(int, line.strip().split(",")))
                for line in f.readlines()]

def shift_tuple(coordinates, i, c):
    return tuple(x + c if i == j else x for (j, x) in enumerate(coordinates))

def neighboring_voxels(coordinates):
    return [shift_tuple(coordinates, i, c)
            for i in range(len(coordinates))
            for c in (-1, 1)]

def unique_sides(voxels):
    sides = set()
    voxels = set(voxels)
    for voxel in voxels:
        for neighbor in neighboring_voxels(voxel):
            if neighbor not in voxels:
                side = Side(inside=voxel, outside=neighbor)
                sides.add(side)
    return sides

def edges_of(side):
    direction = tuple(map(operator.sub, side.outside, side.inside))
    indices = [i for i, d in enumerate(direction) if not d]
    insides = [shift_tuple(side.inside, i, c)
               for i, c in itertools.product(indices, (-1, 1))]
    outsides = [tuple(map(operator.add, inside, direction))
                for inside in insides]
    sides = [Side(inside=i, outside=o) for i, o in zip(insides, outsides)]
    return [Edge(inside=side, outside=other) for other in sides]

def candidate_sides(e):
    '''
    a helpful illustration:
                       | <- concave side
    e.inside.outside <-|-- e.outside.outside
    ---- e.inside ---- e ---- e.outside ----
    e.inside.inside  --|-> e.outside.inside
                       | <- convex side
    '''
    concave = Side(inside=e.outside.outside, outside=e.inside.outside)
    convex = Side(inside=e.inside.inside, outside=e.outside.inside)
    return (concave, e.outside, convex)

def surfaces(sides):
    # partition a set of sides into a list of contiguous surfaces
    surfaces = list() # list of sets of contiguous sides
    while sides:
        surface = {sides.pop()}
        frontier = collections.deque(surface)
        while frontier:
            side = frontier.pop()
            for e in edges_of(side):
                for s in candidate_sides(e):
                    if s in surface:
                        break # already in surface
                    elif s in sides:
                        sides.remove(s); surface.add(s)
                        frontier.append(s)
                        break # only one side per edge
        surfaces.append(surface)
    return surfaces

voxels = parse_voxels("input.txt")
sides = unique_sides(voxels)
print("Part 1:", len(sides))
surfaces = surfaces(sides)
print("Part 2:", max(map(len, surfaces)))
print("- number of surfaces:", len(surfaces))
print("- number of sides for each surface:", list(map(len, surfaces)))
