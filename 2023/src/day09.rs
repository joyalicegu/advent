use crate::Solution;

pub struct Day09;

impl Day09 {
    fn next_value(sequence: &Vec<i64>) -> i64 {
        let mut row: Vec<i64> = sequence.clone();
        let mut result = 0;
        loop {
            if row.iter().all(|&v| v == 0) {
                break; // all zeroes
            }
            for i in 0..(row.len() - 1) {
                row[i] = row[i + 1] - row[i];
            }
            if let Some(last_value) = row.pop() {
                result += last_value;
            } else {
                break; // no more values
            }
        }
        result
    }

    fn prev_value(sequence: &Vec<i64>) -> i64 {
        let reversed = sequence.clone().into_iter().rev().collect();
        Self::next_value(&reversed)
    }
}

impl Solution for Day09 {
    type ParsedInput = Vec<Vec<i64>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let sequences = _parsed_input;
        sequences
            .iter()
            .map(Self::next_value)
            .sum::<i64>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let sequences = _parsed_input;
        sequences
            .iter()
            .map(Self::prev_value)
            .sum::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(Day09::solve_part_one(TEST_INPUT), "114".to_string())
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(Day09::solve_part_two(TEST_INPUT), "2".to_string())
    }
}
