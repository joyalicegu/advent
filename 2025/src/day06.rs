use crate::Solution;
use std::iter::zip;

pub struct Day06;

impl Day06 {
    // TODO helpers
}

impl Solution for Day06 {
    type ParsedInput = Vec<(String, Vec<u64>)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let tmp: Vec<Vec<u64>> = input_lines
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap_or_default())
                    .collect()
            })
            .collect();
        let (rows, cols) = (tmp.len() - 1, tmp[0].len());
        let operands: Vec<Vec<_>> = (0..cols)
            .map(|c| (0..rows).map(|r| tmp[r][c]).collect())
            .collect();
        let ops: Vec<String> = input_lines
            .lines()
            .rev()
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        zip(ops.into_iter(), operands.into_iter()).collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .map(|(op, operands)| {
                if op == "*" {
                    operands.iter().product::<u64>()
                } else if op == "+" {
                    operands.iter().sum::<u64>()
                } else {
                    0
                }
            })
            .sum::<u64>()
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

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(Day06::solve_part_one(TEST_INPUT), "4277556".to_string())
    }
}
