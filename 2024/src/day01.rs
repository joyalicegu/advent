use crate::Solution;
use itertools::Itertools;
use std::iter::zip;

pub struct Day01;

impl Solution for Day01 {
    type ParsedInput = (Vec<i32>, Vec<i32>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .unzip()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (left, right) = _parsed_input;
        zip(left.iter().sorted(), right.iter().sorted())
            .map(|(l, r)| (l - r).abs())
            .sum::<i32>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let (left, right) = _parsed_input;
        let counts = right.iter().counts();
        left.iter()
            .map(|l| l * (*counts.get(l).unwrap_or(&0) as i32))
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(
            Day01::solve_part_one(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            "11".to_string()
        )
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(
            Day01::solve_part_two(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            "31".to_string()
        )
    }
}
