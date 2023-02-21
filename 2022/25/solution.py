import math

DIGITS = { '2': 2, '1': 1, '0': 0, '-': -1, '=': -2, }
STRING = { v : k for k, v in DIGITS.items() }

def to_decimal(snafu):
    result = 0
    for power, digit in enumerate(reversed(snafu)):
        result += DIGITS[digit] * int(math.pow(5, power))
    return result

def to_snafu(decimal):
    result = ""
    while decimal:
        digit = ((decimal + 2) % 5) - 2
        result = STRING[digit] + result
        decimal = (decimal - digit) // 5
    return result

filename = "input.txt"
with open(filename) as f:
    numbers = f.read().splitlines()
decimal_sum = sum((to_decimal(number) for number in numbers))
print(decimal_sum)
assert to_decimal(to_snafu(decimal_sum)) == decimal_sum
print(to_snafu(decimal_sum))
