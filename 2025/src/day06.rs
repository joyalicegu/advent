use crate::Solution;
use std::iter::zip;

pub struct Day06;

impl Day06 {
    fn calculate(problems: &Vec<(String, Vec<u64>)>) -> u64 {
        problems
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
    }

    fn parse_input_one(input_lines: &str) -> Vec<(String, Vec<u64>)> {
        // not very nice
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

    fn parse_input_two(input_lines: &str) -> Vec<(String, Vec<u64>)> {
        // not very nice
        let chars: Vec<Vec<char>> = input_lines
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let (rows, cols) = (chars.len() - 1, chars[0].len());
        let transposed: Vec<String> = (0..cols)
            .map(|c| (0..rows).map(|r| chars[r][c]).collect())
            .collect();
        let mut operands = Vec::new();
        let mut problem = Vec::new();
        for thing in transposed.iter() {
            let trimmed = thing.trim();
            if let Ok(operand) = trimmed.parse::<u64>() {
                problem.push(operand);
            } else {
                operands.push(problem);
                problem = Vec::new();
            }
        }
        operands.push(problem);
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
}

impl Solution for Day06 {
    type ParsedInput = (Vec<(String, Vec<u64>)>, Vec<(String, Vec<u64>)>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        (
            Self::parse_input_one(input_lines),
            Self::parse_input_two(input_lines),
        )
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::calculate(&_parsed_input.0).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::calculate(&_parsed_input.1).to_string()
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

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(Day06::solve_part_two(TEST_INPUT), "3263827".to_string())
    }
}
