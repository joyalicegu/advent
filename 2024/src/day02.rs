use crate::Solution;
use itertools::Itertools;

pub struct Day02;

impl Day02 {
    fn is_all_gradually_increasing(report: &Vec<i32>, min: i32, max: i32) -> bool {
        report
            .iter()
            .tuple_windows()
            .all(|(&a, &b)| a < b && a + min <= b && b <= a + max)
    }

    fn is_all_gradually_decreasing(report: &Vec<i32>, min: i32, max: i32) -> bool {
        report
            .iter()
            .tuple_windows()
            .all(|(&a, &b)| b < a && b + min <= a && a <= b + max)
    }

    fn is_safe(report: &Vec<i32>) -> bool {
        Self::is_all_gradually_increasing(report, 1, 3)
            || Self::is_all_gradually_decreasing(report, 1, 3)
    }

    fn is_gradually_increasing_with_dampener(report: &Vec<i32>, min: i32, max: i32) -> bool {
        let adjacent: Vec<bool> = report
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| a < b && a + min <= b && b <= a + max)
            .collect();
        if adjacent.iter().all(|&b| b) {
            return true;
        }
        let skipping: Vec<bool> = report
            .iter()
            .tuple_windows()
            .map(|(&a, _, &c)| a < c && a + min <= c && c <= a + max)
            .collect();
        for i in 0..report.len() {
            // does it work if we remove the number at index i?
            if (i == 0 || adjacent[..(i - 1)].iter().all(|&b| b))
                && (i == 0 || i == report.len() - 1 || skipping[i - 1])
                && (i == report.len() - 1 || adjacent[(i + 1)..].iter().all(|&b| b))
            {
                return true;
            }
        }
        false
    }

    fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
        let report_reversed = report.clone().into_iter().rev().collect();
        Self::is_gradually_increasing_with_dampener(report, 1, 3)
            || Self::is_gradually_increasing_with_dampener(&report_reversed, 1, 3)
    }
}

impl Solution for Day02 {
    type ParsedInput = Vec<Vec<i32>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .filter(|report| Self::is_safe(report))
            .count()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .filter(|report| Self::is_safe_with_dampener(report))
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(Day02::solve_part_one(TEST_INPUT), "2".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(Day02::solve_part_two(TEST_INPUT), "4".to_string())
    }
}
