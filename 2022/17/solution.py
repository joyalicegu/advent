def parse_jets(filename):
    with open(filename) as f:
        return f.read().strip()

CHAMBER_WIDTH = 7
LEFT_PADDING = 2
BOTTOM_PADDING = 3
CHAMBER_FLOOR = (1 << (CHAMBER_WIDTH + 2)) - 1
EMPTY_ROW = 0b1 << (CHAMBER_WIDTH + 1) | 0b1

def convert_rock(rock):
    lines = rock.splitlines()
    rock_width = len(lines[0])
    shift = CHAMBER_WIDTH - LEFT_PADDING - rock_width + 1
    return [int(line.replace('#', '1').replace('.', '0'), 2) << shift
              for line in lines]

ROCKS = list(map(convert_rock,
"""####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
""".split('\n\n')))

class Rock:
    def __init__(self, index, level):
        self.lines = list(reversed(ROCKS[index]))
        self.level = level # of lowest point in rock
        self.stopped = False

    def collision(self, chamber):
        for i, line in enumerate(self.lines): # bottom to top
            y = self.level + i
            row = chamber[y] if y < len(chamber) else EMPTY_ROW
            if row & line:
                return True
        return False

    def move_horizontal(self, direction, chamber):
        old_lines = self.lines
        if direction == '>':
            self.lines = [line >> 1 for line in self.lines]
        elif direction == '<':
            self.lines = [line << 1 for line in self.lines]
        if self.collision(chamber):
            self.lines = old_lines # restore
            return False
        return True

    def move_down(self, chamber):
        self.level -= 1
        if self.collision(chamber):
            self.level += 1 # restore
            self.stop(chamber)
            return False
        return True

    def stop(self, chamber): # modifies chamber
        self.stopped = True
        for i, line in enumerate(self.lines): # bottom to top
            y = self.level + i
            if y == len(chamber):
                chamber.append(EMPTY_ROW)
            assert y < len(chamber)
            chamber[y] |= line
        return

class Chamber:
    def __init__(self, jets):
        self.chamber = [CHAMBER_FLOOR] # only contains non-empty rows
        self.rock_number = 0
        self.rock_index = 0
        self.jet_index = 0
        self.jets = jets

    def drop_rock(self):
        rock = Rock(self.rock_index, len(self.chamber) + BOTTOM_PADDING)
        while not rock.stopped:
            # move horizontally (if possible)
            direction = self.jets[self.jet_index]
            rock.move_horizontal(direction, self.chamber)
            self.jet_index = (self.jet_index + 1) % len(self.jets)
            # move down (if possible)
            rock.move_down(self.chamber)
        self.rock_number += 1
        self.rock_index = self.rock_number % len(ROCKS)
        return

    def height(self):
        return len(self.chamber) - 1

    def print(self):
        for line in reversed(self.chamber):
            print(bin(line))

def simulate(rock_count, jets):
    chamber = Chamber(jets)
    for _ in range(rock_count):
        chamber.drop_rock()
    return chamber.height()

JETS = parse_jets("input.txt")
print("Part 1:", simulate(2022, JETS))
# print("Part 2:", simulate(1000000000000, JETS))
