use crate::Solution;

pub struct Day07;

impl Day07 {
    // TODO helpers
}

impl Solution for Day07 {
    type ParsedInput = String; // TODO

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
        // TODO
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(Day07::solve_part_one(TEST_INPUT), "3749".to_string())
    }
}
