use crate::Solution;

pub struct Day01;

static TOKENS: [(&'static str, u32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl Day01 {
    fn find_digit(line: &str, reverse: bool) -> u32 {
        let mut result = None; // (index, digit) option
        for (token, digit) in TOKENS.iter() {
            let find_result = if reverse {
                line.rfind(token)
            } else {
                line.find(token)
            };
            if let Some(i) = find_result {
                if result.is_none() {
                    result = Some((i, digit));
                } else if let Some((j, _)) = result {
                    if (reverse && i > j) || (!reverse && i < j) {
                        result = Some((i, digit));
                    }
                }
            }
        }
        *result.unwrap().1
    }
}

impl Solution for Day01 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let mut calibration_values = vec![];
        for line in _parsed_input.lines() {
            let first_digit = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last_digit = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            let calibration_value = first_digit * 10 + last_digit;
            calibration_values.push(calibration_value);
        }
        calibration_values.iter().sum::<u32>().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let mut calibration_values = vec![];
        for line in _parsed_input.lines() {
            let first_digit = Self::find_digit(line, false);
            let last_digit = Self::find_digit(line, true);
            let calibration_value = first_digit * 10 + last_digit;
            calibration_values.push(calibration_value);
        }
        calibration_values.iter().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(
            Day01::solve_part_one(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            "142".to_string()
        )
    }

    #[test]
    fn check_day01_part1_case2() {
        assert_eq!(
            Day01::solve_part_one(
                "two1nine
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "209".to_string()
        )
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(
            Day01::solve_part_two(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "281".to_string()
        )
    }
}
