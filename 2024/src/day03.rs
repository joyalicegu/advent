use crate::Solution;
use regex::Regex;

pub struct Day03;

impl Solution for Day03 {
    type ParsedInput = Vec<(bool, u32, u32)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let inst_re =
            Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<x>[0-9]+),(?<y>[0-9]+)\)")
                .unwrap();
        let mut enabled = true;
        let mut muls = Vec::new();
        for caps in inst_re.captures_iter(input_lines) {
            if caps.name("do").is_some() {
                enabled = true;
            } else if caps.name("dont").is_some() {
                enabled = false;
            } else {
                muls.push((
                    enabled,
                    caps["x"].parse::<u32>().unwrap(),
                    caps["y"].parse::<u32>().unwrap(),
                ))
            }
        }
        muls
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .map(|(_, x, y)| x * y)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .filter(|(enabled, _, _)| *enabled)
            .map(|(_, x, y)| x * y)
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            Day03::solve_part_one(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            "161".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            Day03::solve_part_two(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            ),
            "48".to_string()
        )
    }
}
