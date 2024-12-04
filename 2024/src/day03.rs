use crate::Solution;
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    type ParsedInput = Vec<(u32, u32)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mul_re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        mul_re
            .captures_iter(input_lines)
            .map(|c| {
                let (_, [x, y]) = c.extract();
                (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .map(|(x, y)| x * y)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(Day03::solve_part_one(TEST_INPUT), "161".to_string())
    }
}
