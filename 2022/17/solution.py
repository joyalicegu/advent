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
    def __init__(self, jets, target):
        self.chamber = [CHAMBER_FLOOR] # only contains non-empty rows
        self.rock_number = 0
        self.rock_index = 0
        self.jet_index = 0
        self.jets = jets
        self.target = target # total rocks to drop
        # for cycle detection
        self.offset = 0 # offset to add to height
        self.placements = [] # placement history
        self.heights = [] # height history
        self.skipped = False
        self.cycle_detected = False
        self.cycle_length = None # rocks dropped during cycle
        self.cycle_height = None # height gained during cycle

    def drop_rock(self):
        rock = Rock(self.rock_index, len(self.chamber) + BOTTOM_PADDING)
        x, y = 0, 0
        while not rock.stopped:
            direction = self.jets[self.jet_index]
            self.jet_index = (self.jet_index + 1) % len(self.jets)
            # move horizontally (if possible)
            success = rock.move_horizontal(direction, self.chamber)
            if success:
                x += 1 if direction == '<' else -1
            # move down (if possible)
            success = rock.move_down(self.chamber)
            if success:
                y += 1
        self.placements.append((x, y, self.rock_index, self.jet_index))
        self.heights.append(self.height())
        self.rock_number += 1
        self.rock_index = self.rock_number % len(ROCKS)
        self.detect_cycle()
        return

    def height(self):
        return len(self.chamber) - 1 + self.offset

    def print(self):
        for line in reversed(self.chamber):
            print(bin(line))

    def detect_cycle(self): # called after each rock is dropped
        hare = len(self.placements) - 1
        tortoise = hare // 2
        if hare % 2 != 0 or hare == tortoise:
            return
        if self.placements[tortoise] != self.placements[hare]:
            return
        # assume the tortoise and the hare are both in the cycle
        # assume the distance between them is a multiple of the cycle length
        self.cycle_detected = True
        self.cycle_length = hare - tortoise
        self.cycle_height = self.heights[hare] - self.heights[tortoise]
        # find mu (just for fun)
        mu = 0
        tortoise, hare = 0, tortoise
        while self.placements[tortoise] != self.placements[hare]:
            tortoise += 1
            hare += 1
            mu += 1
        print("beginning of the first cycle:", mu)
        # find lam (just for fun)
        lam = 1
        tortoise, hare = mu, mu + 1
        while self.placements[tortoise] != self.placements[hare]:
            hare += 1
            lam += 1
        print("length of the shortest cycle:", lam)
        return

    def simulate(self):
        while self.rock_number < self.target:
            if self.cycle_detected and not self.skipped: # skip remaining cycles
                remaining_rocks = self.target - self.rock_number
                remaining_cycles = remaining_rocks // self.cycle_length
                self.rock_number += remaining_cycles * self.cycle_length
                self.offset = remaining_cycles * self.cycle_height
                self.skipped = True
            self.drop_rock()
        return self.height()

JETS = parse_jets("input.txt")
print("Part 1:", Chamber(jets=JETS, target=2022).simulate())
print("Part 2:", Chamber(jets=JETS, target=1000000000000).simulate())
