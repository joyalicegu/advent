use crate::Solution;
use itertools::Itertools;

pub struct Day05;

impl Day05 {
    fn fresh_ids(ranges: &Vec<(u64, u64)>, ids: &Vec<u64>) -> Vec<u64> {
        let mut result = Vec::new();
        for &id in ids.iter() {
            for &(a, b) in ranges.iter() {
                if a <= id && id <= b {
                    result.push(id);
                    break;
                }
            }
        }
        result
    }
}

impl Solution for Day05 {
    type ParsedInput = (Vec<(u64, u64)>, Vec<u64>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let (ranges_str, ids_str) = input_lines.split_once("\n\n").unwrap();
        let ranges = ranges_str
            .lines()
            .map(|line| {
                line.split("-")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        let ids = ids_str
            .lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();
        (ranges, ids)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (ranges, ids) = _parsed_input;
        Self::fresh_ids(ranges, ids).len().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(Day05::solve_part_one(TEST_INPUT), "3".to_string())
    }
}
