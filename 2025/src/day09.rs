use crate::Solution;
use itertools::Itertools;

pub struct Day09;

impl Day09 {
    fn largest_area(corners: &Vec<(isize, isize)>) -> isize {
        corners
            .iter()
            .tuple_combinations()
            .map(|(a, b)| ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs())
            .max()
            .expect("corners.len() should be at least 2")
    }
}

impl Solution for Day09 {
    type ParsedInput = Vec<(isize, isize)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let corners = _parsed_input;
        Self::largest_area(corners).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(Day09::solve_part_one(TEST_INPUT), "50".to_string())
    }
}
