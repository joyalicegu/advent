SHAPES = {'A' : 1, 'B' : 2, 'C' : 3,
          'X' : 1, 'Y' : 2, 'Z' : 3}
OUTCOMES = {'X' : 0, 'Y' : 3, 'Z' : 6}

# rock < paper     1 < 2
# paper < scissors 2 < 3
# scissors < rock  3 < 1

# 0 : elf wins
# 3 : draw
# 6 : you win
def outcome_score(elf, you):
    e, y = SHAPES[elf], SHAPES[you]
    return ((y - e + 1) % 3) * 3

def shape_score(elf, outcome):
    e, o = SHAPES[elf], OUTCOMES[outcome] // 3
    return ((o + (e - 1) - 1) % 3) + 1

score1 = 0
score2 = 0
with open("input.txt") as f:
    for line in f.readlines():
        [elf, x] = line.split()
        # part 1
        you = x
        score1 += SHAPES[you]
        score1 += outcome_score(elf, you)
        # part 2
        outcome = x
        score2 += OUTCOMES[outcome]
        score2 += shape_score(elf, outcome)
print(score1)
print(score2)
