import collections
import re

PATTERN = r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"
Sensor = collections.namedtuple('Sensor', ['sensor_x', 'sensor_y', 'beacon_x', 'beacon_y'])

def read_sensors(filename):
    sensors = []
    with open(filename) as f:
        for line in f.read().splitlines():
            match = re.match(PATTERN, line)
            assert match is not None
            sensor = Sensor(
                int(match.group("sensor_x")),
                int(match.group("sensor_y")),
                int(match.group("beacon_x")),
                int(match.group("beacon_y")))
            sensors.append(sensor)
    return sensors

# ranges are inclusive start, inclusive end

def overlap(pair1, pair2): # borrowed from day four
    (a, b), (c, d) = pair1, pair2
    return (b >= c) and (d >= a)

def get_range(s, y, include_beacons=False):
    distance = abs(s.beacon_x - s.sensor_x) + abs(s.beacon_y - s.sensor_y)
    y_distance = abs(s.sensor_y - y)
    if y_distance > distance:
        return None
    x_distance = distance - y_distance
    left = s.sensor_x - x_distance
    right = s.sensor_x + x_distance
    # exclude beacon if beacon is in row
    if not include_beacons and s.beacon_y == y:
        if s.beacon_x == left == right:
            return None
        elif s.beacon_x == left:
            left += 1
        elif s.beacon_x == right:
            right -= 1
    return (left, right)

def merge_ranges(ranges):
    result = []
    ranges = sorted(ranges)
    current = ranges[0]
    for left, right in ranges[1:]:
        if overlap(current, (left, right)):
            current = (min(current[0], left), max(current[1], right))
        else:
            result.append(current)
            current = (left, right)
    result.append(current)
    return result

def get_ranges(sensors, row, include_beacons=False):
    ranges = []
    for sensor in sensors:
        r = get_range(sensor, row, include_beacons=include_beacons)
        if r is None: # empty range
            continue
        ranges.append(r)
    return merge_ranges(ranges)

def count_positions(sensors, y):
    ranges = get_ranges(sensors, y)
    return sum((r - l + 1 for l, r in ranges))

def get_tuning_frequency(open_position):
    return 4000000 * open_position[0] + open_position[1]

def line_intersection(positive, negative):
    # intersection of "y = x + positive" and "y = -x + negative"
    x = (negative - positive) // 2
    y = (negative + positive) // 2
    return (x, y)

def get_open_position(sensors, max_coordinate):
    # find a position bounded by sensor regions
    # assume there's only one such position
    # missing some edge cases
    positive_los = set() # positive slope, lower
    positive_his = set() # positive slope, higher
    negative_los = set() # negative slope, lower
    negative_his = set() # negative slope, higher
    positive = None
    negative = None
    for s in sensors:
        distance = abs(s.beacon_x - s.sensor_x) + abs(s.beacon_y - s.sensor_y) + 1
        lo_y = s.sensor_y + distance
        hi_y = s.sensor_y - distance
        # y = x + b ; x = s.sensor_x, y = lo_y
        # b = lo_y - s.sensor_x
        positive_lo = lo_y - s.sensor_x
        # y = x + b ; x = s.sensor_x, y = hi_y
        # b = hi_y - s.sensor_x
        positive_hi = hi_y - s.sensor_x
        # y = - x + b ; x = s.sensor_x, y = lo_y
        # b = lo_y + s.sensor_x
        negative_lo = lo_y + s.sensor_x
        # y = - x + b ; x = s.sensor_x, y = hi_y
        # b = hi_y + s.sensor_x
        negative_hi = hi_y + s.sensor_x
        # if there's a matching line of the opposite kind of boundary:
        if positive_lo in positive_his: positive = positive_lo
        if positive_hi in positive_los: positive = positive_hi
        if negative_lo in negative_his: negative = negative_lo
        if negative_hi in negative_los: negative = negative_hi
        positive_los.add(positive_lo)
        positive_his.add(positive_hi)
        negative_los.add(negative_lo)
        negative_his.add(negative_hi)
    return line_intersection(positive, negative)

sensors = read_sensors("input.txt")
print("Part 1:", count_positions(sensors, y=2000000))
print("Part 2:", get_tuning_frequency(get_open_position(sensors, 4000000)))
